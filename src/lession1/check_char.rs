// Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
// Ví dụ: let input = 'adbcdaDd'.
// Nhập s = 'a' => in ra kết quả : 2, 'dbcdDd'
// Nhập s = 'd' => in ra kết quả : 4, 'abca'

use std::io::{stdin};

fn main() {
    let str = "adbcdaDd";

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    if input.len() == 0 {
        return;
    }

    let input_ch = input.chars().next().unwrap();

    let mut new_str = String::with_capacity(str.len());
    for c in str.chars() {
        if !input_ch.eq_ignore_ascii_case(&c) {
            new_str.insert(new_str.len(), c);
        }
    }

    println!("original string: {}", str);
    println!("result string: {}", new_str);
}
