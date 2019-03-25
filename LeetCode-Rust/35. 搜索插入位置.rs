fn main() {
    let b = Solution::search_insert(vec![1,2,3,4,5],6);
    println!("{}",b);

}
struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = low + (high - low)/2;
            if nums[mid] < target {
                low = mid + 1;
            } else if nums[mid] > target {
                high = mid;
            } else {
                return  mid as i32;
            }
        }
        return low as i32;
    }
}