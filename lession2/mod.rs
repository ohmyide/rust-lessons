// use std::fs;

// // fn main() {
// //     let text = fs::read_to_string("/Users/chuanjin/personal/rust/greeting/text.txt").unwrap();
// //     println!("{}", text);
// // }

// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans11!");
//     // let message = fs::read_to_string("/Users/chuanjin/personal/rust/greeting/text.txt").unwrap();
//     let _width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), 10, &mut writer).unwrap();
// }

// pub fn say {
//     const X: i32 = 10;

//     fn print() {
//         println!("{}", 42);
//     }
// }

pub fn message() -> String {
    String::from("This is a message from lession2.")
}