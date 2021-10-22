fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(3);
    v1.push(4);
    println!("v1 is {:?}", v1);

    let v2 = vec![1, 2, 3];
    println!("v2 is {:?}", v2);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(elem) => println!("The third element is {}", elem),
        None => println!("There is no third element"),
    }

    for i in &v2 {
        println!("{}", i);
    }

    let mut v2 = vec![1, 2, 3];
    for i in &mut v2 {
        *i += 2;
    }
    println!("v2 is {:?}", v2);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    println!("row is {:?}", row);
}
