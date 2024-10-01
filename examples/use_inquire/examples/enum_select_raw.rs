use std::fmt::{Display, Formatter};

use inquire::{error::InquireResult, Select};

fn main() -> InquireResult<()> {
    let ans: Currency = Select::new("Currency:", Currency::variants().to_vec()).prompt()?;

    match ans {
        Currency::BRL | Currency::USD | Currency::CAD | Currency::EUR | Currency::GBP => {
            bank_transfer();
        }
        Currency::BTC | Currency::LTC => crypto_transfer(),
    }

    Ok(())
}

fn bank_transfer() {
    println!("Bank")
    // ask for bank account
    // transfer funds
}

fn crypto_transfer() {
    println!("Crypto")
    // ask for wallet address
    // transfer funds
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
enum Currency {
    BRL,
    USD,
    CAD,
    EUR,
    GBP,
    BTC,
    LTC,
}

impl Currency {
    fn variants() -> &'static [Currency] {
        &[
            Self::BRL,
            Self::USD,
            Self::CAD,
            Self::EUR,
            Self::GBP,
            Self::BTC,
            Self::LTC,
        ]
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let currency_name = match self {
            Currency::BRL => "Brazilian Real",
            Currency::USD => "US Dollar",
            Currency::CAD => "Canadian Dollar",
            Currency::EUR => "Euro",
            Currency::GBP => "British Pound",
            Currency::BTC => "Bitcoin",
            Currency::LTC => "Litecoin",
        };
        write!(f, "{}", currency_name)
    }
}
