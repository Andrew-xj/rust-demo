enum SpreadsheetCell {
	Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // 遍历 vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in v {
        println!("{}", i);
    }

    // vector + enum
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}