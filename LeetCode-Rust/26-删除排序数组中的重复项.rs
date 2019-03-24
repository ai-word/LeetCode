


fn main() {
    let mut arr = vec![1,2,3,4,4,4,5];
    let a = Solution::remove_duplicates(&mut arr);
    println!("{}",a);
}
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        if  len  < 2 {
            return len as i32;
        }
        let mut count = 0;
        for i in 1..len {
            if nums[count] != nums[i]{
                count += 1;
                nums[count] = nums[i];
            }
        }
        return count as i32 + 1;
    }
}