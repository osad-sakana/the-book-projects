enum Coin {
    OneYen,
    FiveYen,
    TenYen,
    FiftyYen,
    HundredYen,
    FiveHundredYen,
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::OneYen => 1,
            Coin::FiveYen => 5,
            Coin::TenYen => 10,
            Coin::FiftyYen => 50,
            Coin::HundredYen => 100,
            Coin::FiveHundredYen => 500,
        }
    }
}

fn main(){
    let coin_list = [
        Coin::OneYen,
        Coin::FiveYen,
        Coin::TenYen,
        Coin::FiftyYen,
        Coin::HundredYen,
        Coin::FiveHundredYen,
    ];

    for coin in &coin_list {
        println!("The value of the coin is: {} yen", coin.value());
    }
    let total_value: u32 = coin_list.iter().map(|coin| coin.value()).sum();

    println!("The total value of all coins is: {} yen", total_value);
}
