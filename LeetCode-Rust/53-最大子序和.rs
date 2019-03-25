impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = nums[0];
        for num in nums {
            if sum < 0 {
                sum = num
            } else {
                sum += num
            }
            if sum > max_sum{
                max_sum = sum
            }
        }
        return max_sum
    }
}