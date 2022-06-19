// use std::fs;

// // fn main() {
// //     let text = fs::read_to_string("/Users/chuanjin/personal/rust/greeting/text.txt").unwrap();
// //     println!("{}", text);
// // }

// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};
// fn say;
// fn main() {
//     // use say::print;
//     // pub mod say::print;

//     say();
//     // let stdout = stdout();
//     // // let message = String::from("Hello fellow Rustaceans!");
//     // let message = fs::read_to_string("/Users/chuanjin/personal/rust/greeting/text.txt").unwrap();
//     // let _width = message.chars().count();

//     // let mut writer = BufWriter::new(stdout.lock());
//     // say(message.as_bytes(), 10, &mut writer).unwrap();
// }


// mod nation {
//     pub mod government {
//         pub fn govern() {
//             println!("{}", 42);
//         }
//     }
// }

// use nation::government::govern;
// mod lession0::say;

// pub mod lession1;
pub mod lession2;
fn main() {
    println!("This is the main module.");
    println!("{}", lession2::message());
}