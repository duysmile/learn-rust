// Bài tập 1: Cho 2 mảng có các phần tử là số nguyên dương, kiểm tra mảng này có phải là mảng con của mảng kia không ?(yêu cầu đúng thứ tự của các phần tử)
// let org_arr = [1, 2,3,5,6,8, 10, 11];
// let sub_arr = [6,8,10];

fn main() {
    let org_arr = [1, 2, 3, 4, 5, 6];
    let sub_arr = [1, 2, 4];
    // let sub_arr = [1, 2, 3];

    let mut i:usize = 0;
    let mut j:usize = 0;
    loop {
        if i >= org_arr.len() {
            break
        }

        if j >= sub_arr.len() {
            break
        }

        if org_arr[i] == sub_arr[j] {
            j += 1;
        } else {
            j = 0;
        }
        i += 1
    }

    if j == sub_arr.len() {
        println!("{:?} is sub-array of {:?}", sub_arr, org_arr)
    } else {
        println!("{:?} is NOT sub-array of {:?}", sub_arr, org_arr)
    }
}
