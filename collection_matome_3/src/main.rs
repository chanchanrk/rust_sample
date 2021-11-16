use std::collections::BTreeMap;
use std::io;

fn main() {

    let mut map = BTreeMap::<String, Vec<String>>::new();

    loop {
        println!("add 雇用者名 to 部署名、または exit：");

        let mut instr = String::new();

        io::stdin()
            .read_line(&mut instr)
            .expect("Failed to read line");
        instr = instr.trim().to_string();

        if instr == "exit" {
            break;
        }

        let employee = String::from(nth_word(&instr, 2)).to_string();
        let department = String::from(nth_word(&instr, 4)).to_string();

        if department != "" && employee != "" {

            match map.get_mut(&department) {
                Some(v) => {
                    v.push(employee);
                },
                None => {
                    let v = vec![employee];
                    map.insert(department, v);
                }
            };
        } else {
            println!("!ERROR blank department or employee {}, {}", &department, &employee);
        }
    }

    for value in map.values_mut() {
        value.sort();
    }

    loop {
        println!("部署名 or 入力なしで全て or exit：");

        let mut instr = String::new();

        io::stdin()
            .read_line(&mut instr)
            .expect("Failed to read line");
        instr = instr.trim().to_string();

        if instr == "exit" {
            break;
        }

        if instr != "" {

            match map.get_mut(&instr) {
                Some(value) => {
                    for employee in value.iter() {
                        println!("部署：{} 雇用者：{}", instr, employee);
                    }
                },
                None => {
                    println!("!ERROR 該当する部署がありません {}", instr);
                }
            };
        } else {
            for (key, value) in &map {
                for employee in value.iter() {
                    println!("部署：{} 雇用者：{}", key, employee);
                }
            }
        }
    }

}

fn nth_word(s: &str, pos: u8) -> &str {
    let bytes = s.as_bytes();

    let mut count:u8 = 0;
    let mut lastpos = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count += 1;
            if count >= pos {
                return &s[lastpos..i];
            }
            lastpos = i + 1;
        }
    }

    if count < pos - 1 {
        ""
    } else {
        &s[lastpos..s.chars().count()]
    }
}
