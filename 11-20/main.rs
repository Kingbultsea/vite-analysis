enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
}

fn main() {
    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Float(10.12), SpreadsheetCell::Text(String::from("String text"))];
    for i in &row {
        println!("{:?}", i);
        match i {
            SpreadsheetCell::Int(i2) => {
                println!("{}", i2);
            },
            _ => {
                println!("..");
            }
        };

        if let SpreadsheetCell::Float(value) = i {
            println!("{}", value);
        }
    }
}