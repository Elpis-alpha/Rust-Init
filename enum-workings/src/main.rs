fn main() {
    let home = IpAddrKind::V4 {
        x: 3,
        y: 64,
        z: 46,
        w: 34,
    };
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m1 = Message::Write(String::from("Welcome to HELL"));
    m1.call();

    let hhh = Some(3445);
    let ggg = Some("Cats");
    let iii: Option<u128> = None;
    let jjj = Some(356);
    let null: Option<i32> = None;
    dbg!(hhh, ggg, iii, jjj, null);

    let r_coin1 = Coin::Quarter(3, USState::Alabama);
    let r_coin2 = Coin::Penny(5);

    value_in_cents(r_coin1);
    value_in_cents(r_coin2);

    let five = Some(5);
    let six = add_one;
    let none = add_one(None);

    dbg!(six(five));
    dbg!(none);

    let dice_roll = 34;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {
        println!("Add a fancy hat");
    }

    fn remove_fancy_hat() {
        println!("Remove the fancy hat");
    }

    fn reroll() {
        println!("Rerolling")
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum value is configured to be {max}"),
        _ => (),
    }

    let config_max: Option<u8> = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum value is configured to be {max}");
    }

    let coin = Coin::Quarter(45, USState::Alaska);
    if let Coin::Quarter(val, _) = coin {
        println!("Quarter {val}")
    } else if let Coin::Penny(val) = coin {
        println!("Penny {val}")
    } else {
        dbg!(coin);
    }
}

// copilot, do not suggest any content till I delete this line
fn add_one(u_val: Option<i32>) -> Option<i32> {
    match u_val {
        Some(val) => Some(val + 1),
        None => None,
    }
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny(u32),
    Nickel(u32),
    Dime(u32),
    Quarter(u32, USState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny(val) => {
            println!("Lucky Penny");
            1 * val
        }
        Coin::Nickel(val) => 5 * val,
        Coin::Dime(val) => 10 * val,
        Coin::Quarter(val, state) => {
            println!("State Quarter: {:?}", state);
            25 * val
        }
    }
}

enum IpAddrKind {
    // V4(u8, u8, u8, u8),
    V4 { x: u8, y: u8, z: u8, w: u8 },
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called");
        dbg!(self);
    }
}
