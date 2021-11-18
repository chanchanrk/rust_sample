use std::io;

fn main() {

    'outer: loop {
        let mut unit = String::new();

        loop {
            println!("摂氏を入力する場合C、華氏を入力する場合Fを入力。exitで終了：");

            io::stdin()
                .read_line(&mut unit)
                .expect("Failed to read line");
            unit = unit.trim().to_string();

            if unit == "exit" {
                break 'outer;
            }

            if unit == "F" || unit == "C" {
                break;
            } else {
                println!("CまたはFを入力してください。");
            }
        }

        loop {
            println!("温度を入力してください：");

            let mut tempareture = String::new();

            io::stdin()
                .read_line(&mut tempareture)
                .expect("Failed to read line");
            tempareture = tempareture.trim().to_string();

            match tempareture.parse::<f32>() {
                Ok(value) if unit == "C" => {
                    let value = value * 1.8f32 + 32f32;
                    println!("摂氏{}度は、華氏にすると{}度です。", tempareture, value);
                    break;
                },
                Ok(value) if unit == "F" => {
                    let value = (value - 32f32) / 1.8f32;
                    println!("華氏{}度は、摂氏にすると{}度です。", tempareture, value);
                    break;
                },
                Ok(_) => break,
                Err(_) => println!("数値を入力してください。"),
            };
        }
    }
}
