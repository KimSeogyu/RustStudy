fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word = first_word(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word = first_word(my_string_literal);
}

// fn main() {
//     // let mut s = String::from("hello");

//     // s.push_str(", world!");

//     // println!("{}", s);

//     // let s = String::from("hello");

//     // let s2 = s;

//     // println!("{}", s);

//     // let x = 5;
//     // let y = x;
//     // println!("x = {}, y = {}", x, y);

//     // let s = String::from("Hello");

//     // takes_ownership(s);

//     // let x = 5;

//     // makes_copy(x);

//     // println!("{}", x);

//     let s1 = String::from("hello");
//     let len = strlen(&s1);

//     println!("The length of '{}' is {}.", s1, len);

//     let mut hello = String::from("Hello");
//     change(&mut hello);
// }

// fn strlen(s: &String) -> usize {
//     s.len()
// }

// fn change(some_str: &mut String) {
//     some_str.push_str(", world");
// }

// // fn makes_copy(some_int: i32) {
// //     println!("{}", some_int);
// // }

// // fn takes_ownership(some_str: String) {
// //     println!("{}", some_str);
// // }

// // fn gives_ownership() -> String {
// //     let some_string = String::from("hello");

// //     some_string
// // }

// // fn takes_and_gives_back(a_str: String) -> String {
// //     a_str
// // }
