use std::io;

fn main() {

    loop {
        println!("フィボナッチ数列の何番目を得ますか？：");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        number = number.trim().to_string();

        match number.parse::<u32>() {
            Ok(num) => {
                let mut counter = 0;
                let mut prev_prev_fibonacci = 0;
                let mut prev_fibonacci = 0;
                loop {
                    let mut fibonacci;
                    if counter == 0 || counter == 1{
                        fibonacci = counter;
                    } else {
                        fibonacci = prev_prev_fibonacci + prev_fibonacci;
                    }
                    if counter == (num + 1) {
                        println!("フィボナッチ数列の{}番目は、{}です。", number, fibonacci);
                        // println!("{}, {}, {}", fibonacci, prev_fibonacci, prev_prev_fibonacci);
                        break;
                    }
                    prev_prev_fibonacci = prev_fibonacci;
                    prev_fibonacci = fibonacci;

                    // println!("{}, {}, {}", fibonacci, prev_fibonacci, prev_prev_fibonacci);

                    counter += 1;
                }
            },
            Err(_) => {
                println!("終了します");
                break;
            },
        };
    }
}
