use std::collections::HashMap;
fn main() {
    let mut v = vec![12, 5, 500, 3, 3, 2, -10, 5, 8, 10, 5, 5];
    let mut count = 0;
    let mut sum = 0;
    for d in v.iter() {
        sum += d;
        count += 1;
    }
    // 平均
    println!("average is {}", sum / count);

    v.sort();
    let vlen = v.len();
    // 中央値
    if vlen % 2 == 0 {
        println!("medium is {}", (v[vlen / 2] + v[vlen / 2 + 1]) / 2);
    } else {
        println!("medium is {}", v[vlen / 2 + 1]);
    }

    let mut map = HashMap::new();
    for d in v.iter() {
        let count = map.entry(d).or_insert(0);
        *count += 1;
    }

    let mut maxvalue = 0;
    let mut maxkey = 0;
    for (key, value) in &map {
        if maxvalue < *value {
            maxvalue = *value;
            maxkey = **key;
        }
    }
    // 最頻値
    println!("mode is {}", maxkey);
}
