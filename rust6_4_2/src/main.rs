fn main() {
    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    match coin {
        // {:?}州のクォーターコイン
        Coin::Quarter(ref state) => println!("State quarter from 1 {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from 2 {:?}!", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

