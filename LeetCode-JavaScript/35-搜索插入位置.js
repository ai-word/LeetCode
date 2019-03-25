/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var searchInsert = function(nums, target) {
    let low = 0
    let high = nums.length
    while (low < high){
        let mid = parseInt(low + (high-low)/2)
        if (nums[mid] < target){
            low = mid +1
        } else if (nums[mid] > target){
            high = mid
        } else {
            return mid
        }
    }
    return low
};