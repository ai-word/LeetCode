fn main() {
   let str = String::from("abcabcbb");
    println!("{}",Solution::length_of_longest_substring(str));
}
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let seq: Vec<char> = s.chars().collect();
        let mut sum_str:Vec<char> = vec![];
        let mut i = 0;
        let mut k = 0;
        let mut max = 0;
        let len = s.len();
        while i < len && k < len {
            if index_str(seq[k],sum_str.clone()) == -1 {
                sum_str.push(seq[k]);
                k += 1;
            } else {
                i += 1;
                k = i;
                sum_str = vec![];
            }
            if max < sum_str.len(){
                max = sum_str.len()
            }
        }
       return max as i32;
    }
}
fn index_str(c:char, vec:Vec<char>)-> i32{
    for item in vec {
        if item == c {
            return 1;
        }
    }
    return -1
}
