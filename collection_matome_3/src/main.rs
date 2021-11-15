use std::collections::HashMap;
use std::io;
//use itertools::Itertools;

fn main() {

    let mut map = HashMap::<String, Vec<String>>::new();

    loop {
        println!("add 部署名 to 雇用者名、または exit：");

        let mut instr = String::new();

        io::stdin()
            .read_line(&mut instr)
            .expect("Failed to read line");
        instr = instr.trim().to_string();

        let department = String::from(instr[..].to_string());
        let department = String::from(nth_word(&department, 2)).to_string();
        let employee = String::from(instr[..].to_string());
        let employee = String::from(nth_word(&employee, 4)).to_string();

        match &department[..] {
            "exit" => {
                //println!("match {}", &department[..]);
                break;
            },
            _ => {
                //println!("unmatch {}", &department[..]);
                //println!("values {}, {}", &department[..], &employee[..]);

                if &department[..] != "" && &employee[..] != "" {

                    match map.get_mut(&department) {
                        Some(v) => {
                            v.push(employee);
                            v.sort();
                        },
                        None => {
                            let v = vec![employee];
                            map.insert(department, v);
                        }
                    };
                } else {
                    println!("!ERROR blank department or employee {}, {}", &department[..], &employee[..]);
                }
            },
        }
    }
    
    // loop {
    //     println!("部署名 or 入力なしで全て or exit：");

    //     let mut instr = String::new();

    //     io::stdin()
    //         .read_line(&mut instr)
    //         .expect("Failed to read line");

    //     let department = String::from(instr[..].to_string());

    //     match &department[0..4] {
    //         "exit" => {
    //             println!("match {}", &department[0..4]);
    //             break;
    //         },
    //         _ => {
    //             println!("unmatch {}", &department[0..4]);
                
    //         },
    //     }
    // }

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
