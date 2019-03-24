impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut rev =vec![-1i32;2];
        for i in 0..nums.len()  {
            let mut a = target - nums[i];
            for k in i+1..nums.len() {
                if a == nums[k] {
                    rev[0]=k as i32;
                    rev[1]=i as i32;
                    return rev 
                }
            }
        }
        rev
    }
}
