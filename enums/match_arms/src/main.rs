fn main() {
    let new_coin = Coin::Athanni(IndState::Assam);
    let value: u8 = value_in_coins(new_coin);
    println!("value of the coin is {value}");

    
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
// It will panic as match is exahustive i.e., all the variants of enums
// must be cover while matching. In this case None is not present.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

fn

fn value_in_coins(coin: Coin) -> u8 {
    match coin {
        Coin::Ekanni => 1,
        Coin::Duanni => 2,
        Coin::Chawanni => 4,
        Coin::Athanni(state) => {
            println!("State quarter from {state:?}");
            8
        }
    }
}

#[derive(Debug)]
enum IndState {
    Andhra_Pradesh,
    Arunachal_Pradesh,
    Assam,
    Bihar,
}

enum Coin {
    Ekanni,
    Duanni,
    Chawanni,
    Athanni(IndState),
}
