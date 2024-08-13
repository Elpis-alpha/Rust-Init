use std::collections::HashMap;

fn main() {
    let mut v = vec![
        SpreadsheetCell::Integer(4),
        SpreadsheetCell::Text(String::from("cats")),
    ];

    v.push(SpreadsheetCell::Integer(5));
    v.push(SpreadsheetCell::Integer(6));
    v.push(SpreadsheetCell::Integer(7));
    v.push(SpreadsheetCell::Integer(8));

    let third = &v[2];
    println!("{third:#?}");

    let eight: Option<&SpreadsheetCell> = v.get(7);
    if let Some(val) = eight {
        println!("{val:#?}")
    }
    match eight {
        Some(val) => println!("{val:#?}"),
        None => println!("None"),
    }

    for ele in &v {
        match ele {
            SpreadsheetCell::Integer(val) => println!("Integer: {val}"),
            SpreadsheetCell::Float(val) => println!("Float: {val}"),
            SpreadsheetCell::Text(val) => {
                println!("Text: {val}");

                let data = "init conts";
                let mut s = data.to_string();

                s.push_str("cats");
                println!("{s}");

                let s1 = String::from("Hello, ");
                let s2 = String::from("world!");
                // let s3 = s1.clone() + &s2;
                let s3 = format!("{s1}{s2}");

                println!("s1-({s1}) + s2-({s2}) = s3-({s3})");
            },
        }
    }

    v.push(SpreadsheetCell::Float(0.04));
    println!("{:#?}", v);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 40);
    scores.insert(String::from("Red"), 23);
    scores.entry(String::from("Blues")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("{scores:#?}");

    
    let text = "billy has two goats each in each of his two gardens";
    let splitted_text = text.split_whitespace();
    let mut map = HashMap::new();

    for tx in splitted_text {
        let count = map.entry(tx).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

#[derive(Debug)]
enum SpreadsheetCell {
    Integer(i32),
    Float(f64),
    Text(String),
}
