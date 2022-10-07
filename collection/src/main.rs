use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    s.push_str("string");
    let s2 = "bar";

    s.push_str(s2);

    println!("s2 is {}", s2);
    println!("{}", s);

    s += ", world";

    println!("{}", s);

    let s1 = "tic";
    let s2 = "tac";
    let s3 = "toe";

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    let hello = "hello";

    for (i, c) in hello.chars().enumerate() {
        println!("{}, {}", i, c);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec!["Blue".to_string(), "Yellow".to_string(), "Red".to_string()];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
    let team_name = "Blue".to_string();
    let score = scores.get(&team_name);
    println!("{}", score.unwrap());

    scores.entry(&"Blue".to_string()).or_insert(&32);
}

// fn main() {
//     let v: Vec<i32> = Vec::new();
//     println!("{:?}", v);

//     let v = vec![1, 2, 3, 4, 5];

//     let third: Option<&i32> = v.get(100);
//     println!("{:#?}", third);

//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }

//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//     }

//     for i in &v {
//         println!("{}", i);
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];

//     println!("{:#?}", row);
// }

// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }
