#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{row:?}");

    let _v: Vec<i32> = Vec::new(); // pusty vector
    let _v1 = vec![1, 2, 3]; // vector z początkowymi wartościami

    let mut v2 = Vec::new();
    v2.push(6); // na koniec
    v2.push(7); // na koniec
    println!("{v2:?}");

    // Odczytywanie przez indeksowanie lub metodę get
    let mut v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // Zwykła iteracja
    for i in &v3 {
        println!("{i}");
    }

    // Iteracja mutowalna
    for i in &mut v3 {
        *i += 50;
    }
    println!("{v3:?}");
}