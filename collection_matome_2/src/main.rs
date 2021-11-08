fn main() {
    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("apple"));
}

fn pig_latin(s: &str) -> String {

    let vec = "aiueo".chars().collect::<Vec<_>>();

    let slice = s.chars().nth(0);
    if let Some(v) = slice {
        if vec.contains(&v) {
            let mut a = String::from(s);
            a += "-hay";
            a
        } else {
            let mut a = String::from(s);
            a = a[1..].to_string();
            a = a + "-" + &v.to_string() + "ay";
            a
        }
    } else {
        "error".to_string()
    }

}