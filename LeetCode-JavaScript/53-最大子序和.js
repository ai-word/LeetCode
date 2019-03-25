/**
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
    let sum = 0
    let max_sum = nums[0]
    for (let i = 0;i < nums.length;i++) {
        if (sum < 0) {
            sum = nums[i]
        } else {
            sum += nums[i]
        }
        if (sum > max_sum) {
            max_sum = sum
        }
    }
    return max_sum
};