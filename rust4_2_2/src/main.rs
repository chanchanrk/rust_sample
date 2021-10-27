fn main() {
    let s1 = no_dangle();

    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
