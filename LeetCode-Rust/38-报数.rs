
use std::char::from_digit;
fn main() {
    let b = Solution::count_and_say(4);
    println!("{}",b);

}
struct Solution;
// 正常做法
//impl Solution {
//    pub fn count_and_say(n: i32) -> String {
//        let mut res = vec!['1'];
//        for _ in 0..n-1 {
//            let mut temp = Vec::new();
//            let mut i = 0_usize;
//            while i < res.len() {
//                let mut j = i + 1;
//                while j < res.len() && res[j] == res[i] {
//                    j += 1;
//                }
//                temp.push(from_digit((j - i) as u32, 10).unwrap());
//                temp.push(res[i]);
//                i = j;
//            }
//            res = temp;
//        }
//        res.iter().collect()
//    }
//}
// 递归做法
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        } else {
            let mut rvecs = vec![];
            let str = &*(Solution::count_and_say(n-1));
            let mut vecs = vec![];
            let mut count:u32 = 1;
            for char in str.chars() {
                vecs.push(char);
            }
            for i in 0..vecs.len() {
                if i+1 < vecs.len() && vecs[i] == vecs[i+1]{
                    count += 1;
                } else {
                    rvecs.push(from_digit(count, 10).unwrap());
                    rvecs.push(vecs[i]);
                    count = 1;
                }
            }
            return rvecs.iter().collect();
        }
    }
}