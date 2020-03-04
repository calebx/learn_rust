#[derive(Debug)]
enum IpAddrType {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dim,
    Quarter(String),
}

#[allow(dead_code)]
impl Coin {
    fn value(&self) -> u32 {
        // match works a lot for enum types
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dim => 10,
            Coin::Quarter(_) => 25,
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

// Enums in Rust are most similar to Haskell
fn main() {
    let v4 = IpAddrType::V4;
    let v6 = IpAddrType::V6;
    println!("v4: {:?}, v6: {:?}", v4, v6);

    let v4 = IpAddr::V4(172, 0, 0, 1);
    let v6 = IpAddr::V6("::1".into());
    println!("new v4: {:?}, new v6 {:?}", v4, v6);

    let penny = Coin::Penny;
    println!("value of penny is {}", penny.value());
    let dim = Coin::Dim;
    println!("value of dim is {}", dim.value());

    let oi = Some(5);
    println!("value of option int add one is {:?}", plus_one(oi));

    // if let some times is the replacement of match
    let quarter = Coin::Quarter("Califonia".into());
    if let Coin::Quarter(state) = quarter {
        println!("{:?}", state);
    }
}
