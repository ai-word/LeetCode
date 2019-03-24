
fn main() {
    let mut arr = vec![1,2,3,4,4,4,5];
    let a = Solution::remove_element(&mut arr,4);
    println!("{}",a);
}
struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0
        }
        let mut  i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }
        return i as i32;
    }
}