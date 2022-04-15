use std::collections::HashMap;
use std::io;

fn main() {
    println!("Please enter your address");

    let mut address = String::new();
    io::stdin().read_line(&mut address).expect("Failed to read line");

    let mut balances = HashMap::new();
    balances.insert(Token::USDT, 1000.0);

    let mut wallet = Wallet::new(address, balances);

    println!("Your wallet has been created");

    loop {
        println!("\n1 - Check your balance");
        println!("2 - See the market");
        println!("3 - Buy token");
        println!("4 - Sell token");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => wallet.check_balance(),
            2 => Token::show_market(),
            3 => buy_token(&mut wallet),
            4 => sell_token(&mut wallet),
            _ => println!("Invalid option"),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum Token {
    SOL,
    DOT,
    ETH,
    BTC,
    USDT,
}

impl Token {
    fn show_market() {
        println!("BTC: price: {}", Self::price(&Token::BTC));
        println!("ETH: price: {}", Self::price(&Token::ETH));
        println!("SOL: price: {}", Self::price(&Token::SOL));
        println!("DOT: price: {}", Self::price(&Token::DOT));
        println!("USDT: price: {}", Self::price(&Token::USDT));
    }

    fn from_str(input: &str) -> Self {
        match input.to_lowercase().as_str() {
            "sol" => Token::SOL,
            "dot" => Token::DOT,
            "btc" => Token::BTC,
            "eth" => Token::ETH,
            "usdt" => Token::USDT,
            _ => Token::BTC,
        }
    }

    fn price(&self) -> f64 {
        match self {
            Token::SOL => 34.0,
            Token::BTC => 30000.0,
            Token::ETH => 1000.0,
            Token::DOT => 8.0,
            Token::USDT => 1.0,
        }
    }

    fn buy(self, amount: f64, balances: &mut HashMap<Token, f64>) {
        let usdt_balance = balances.get(&Token::USDT).unwrap();
        let total_price = self.price() * amount;

        if usdt_balance >= &total_price {
            balances.insert(Token::USDT, usdt_balance - total_price);
            *balances.entry(self).or_insert(0.0) += amount;
            println!("Transaction successful!");
        } else {
            println!("Insufficient balance. Transaction declined.");
        }
    }

    fn sell(self, amount: f64, balances: &mut HashMap<Token, f64>) {
        let token_balance = balances.get(&self).unwrap_or(&0.0);

        if token_balance >= &amount {
            let usdt_balance = balances.get(&Token::USDT).unwrap();
            let total_price = self.price() * amount;
            balances.insert(self, token_balance - amount);
            balances.insert(Token::USDT, usdt_balance + total_price);
            println!("Transaction successful!");
        } else {
            println!("Invalid amount. Transaction declined.");
        }
    }
}

struct Wallet {
    address: String,
    balances: HashMap<Token, f64>,
}

impl Wallet {
    fn new(address: String, balances: HashMap<Token, f64>) -> Self {
        Self { address, balances }
    }

    fn check_balance(&self) {
        println!("{:?}", self.balances);
    }
}

fn buy_token(wallet: &mut Wallet) {
    println!("Which token do you want to buy?");
    Token::show_market();

    let mut token_name = String::new();
    io::stdin().read_line(&mut token_name).expect("Failed to read line");
    let token = Token::from_str(token_name.trim());

    println!("Enter the amount:");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line");
    let amount: f64 = amount.trim().parse().expect("Invalid input");

    token.buy(amount, &mut wallet.balances);
}

fn sell_token(wallet: &mut Wallet) {
    println!("Which token do you want to sell?");
    wallet.check_balance();

    let mut token_name = String::new();
    io::stdin().read_line(&mut token_name).expect("Failed to read line");
    let token = Token::from_str(token_name.trim());

    println!("Enter the amount:");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line");
    let amount: f64 = amount.trim().parse().expect("Invalid input");

    token.sell(amount, &mut wallet.balances);
}
