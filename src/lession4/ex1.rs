/////////////////////////////////////
// Bài 1
// Sửa code để compile thành công liên quan tới vấn đề lifetime
/////////////////////////////////////

use std::io;
fn main() {
    let mut trimmed_text: String;
    let input = loop {
        let mut input_text = String::new();
        println!("Type instruction in the format Add <name> to <department>:");
        io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
        trimmed_text = input_text.trim().to_string();
        let input: Vec<&str> = trimmed_text.split(" ").collect();
        if input[0] == "Add" && input[2] == "to" {
            break input
        } else {
            println!("Invalid format.");
        }
    };
    println!("{:?}", input);
}
