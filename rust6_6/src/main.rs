fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three 1"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three 2");
    }
}
