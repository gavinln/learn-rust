fn main() {
    let c = Coin::Penny;
    println!("c is {:?}", c);
    println!("value is {}", value_in_cents(c));

    let c2 = Coin2::Quarter(UsState::Alaska);

    println!("c2 is {:?}", c2);
    println!("value is {}", value_in_cents2(&c2));

    if let Coin2::Quarter(state) = c2 {
        println!("c2 is a state quarter from {:?}", state);
    } else {
        println!("Not a quarter");
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: &Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
