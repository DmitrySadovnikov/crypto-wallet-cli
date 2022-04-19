use std::collections::HashMap;
use std::io;

fn main() {
    println!("📍 Please enter your address:");

    let mut address = String::new();
    io::stdin().read_line(&mut address).expect("Failed to read line");

    let mut balances = HashMap::new();
    balances.insert(Token::USDT, 1000.0);

    let mut wallet = Wallet::new(address.trim().to_string(), balances);

    println!("🎉 Your wallet has been created!");

    loop {
        println!("1️⃣ - Check your balance");
        println!("2️⃣ - Show wallet address");
        println!("3️⃣ - See the market");
        println!("4️⃣ - Buy token");
        println!("5️⃣ - Sell token");
        println!("6️⃣ - Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => wallet.check_balance(),
            2 => wallet.show_address(),
            3 => Token::show_market(),
            4 => buy_token(&mut wallet),
            5 => sell_token(&mut wallet),
            6 => {
                println!("👋 Exiting...");
                break;
            }
            _ => println!("❌ Invalid option. Please try again."),
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
        println!("📈 Market Prices:");
        println!("BTC: price: ${}", Self::price(&Token::BTC));
        println!("ETH: price: ${}", Self::price(&Token::ETH));
        println!("SOL: price: ${}", Self::price(&Token::SOL));
        println!("DOT: price: ${}", Self::price(&Token::DOT));
        println!("USDT: price: ${}", Self::price(&Token::USDT));
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
            println!("✅ Transaction successful!");
        } else {
            println!("❌ Insufficient balance. Transaction declined.");
        }
    }

    fn sell(self, amount: f64, balances: &mut HashMap<Token, f64>) {
        if self == Token::USDT {
            println!("❌ Cannot sell USDT directly.");
            return;
        }

        let token_balance = balances.get(&self).unwrap_or(&0.0);

        if token_balance >= &amount {
            let usdt_balance = *balances.get(&Token::USDT).unwrap();
            let total_price = self.price() * amount;
            balances.insert(self, token_balance - amount);
            balances.insert(Token::USDT, usdt_balance + total_price);
            println!("✅ Transaction successful!");
        } else {
            println!("❌ Invalid amount. Transaction declined.");
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

    fn show_address(&self) {
        println!("Wallet address: {}", self.address);
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
    let amount: f64 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid input. Please enter a valid number.");
            return;
        }
    };

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
    let amount: f64 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Invalid input. Please enter a valid number.");
            return;
        }
    };

    token.sell(amount, &mut wallet.balances);
}
