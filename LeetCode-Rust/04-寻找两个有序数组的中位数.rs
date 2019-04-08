fn main() {
    println!("{}",Solution::find_median_sorted_arrays(vec![1,3],vec![2]))
}
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut nums_1 = nums1.clone();
        let mut nums_2 = nums2.clone();
        if len1 == 0 {
            if len2 == 1 {
                return nums2[0] as f64;
            } else {
                if len2 % 2 == 0 {
                    let v = len2 / 2;
                    return ((nums2[v] + nums2[v-1]) as f64)/2.0;
                } else {
                    let v = len2 / 2;
                    return nums2[v] as f64;
                }
            }
        }
        if len2 == 0 {
            if len1 == 1 {
                return nums1[0] as f64;
            } else {
                if len1 % 2 == 0 {
                    let v = len1 / 2;
                    return ((nums1[v] + nums1[v-1]) as f64)/2.0;
                } else {
                    let v = len1 / 2;
                    return nums1[v] as f64;
                }
            }
        }
        let mut limit = 0;
        let mut times = 0;
        let mut  flag = 0;
        let len_sum = len1 + len2;
        if len_sum % 2 == 0 {
            flag = 2;
//            循环的次数
            limit = len_sum / 2 + 1
        } else {
            flag = 1;
            limit = len_sum / 2 + 1
        }
        let mut ans = vec![];
        while times < limit {
            let mut l1 = nums_1.len();
            let mut l2 = nums_2.len();
            let l = ans.len();
            if l1 != 0 && l2 != 0 {
                if nums_1[0] > nums_2[0] {
                    let el = nums_2[0];
                    ans.push(el);
                    nums_2.remove(0 as usize);
                } else {
                    let el = nums_1[0];
                    ans.push(el);
                    nums_1.remove(0 as usize);
                }
            } else if l1 == 0 && l2 != 0 {
                let el = nums_2[0];
                ans.push(el);
                nums_2.remove(0 as usize);
            } else if l2 == 0 && l1 != 0 {
                let el = nums_1[0];
                ans.push(el);
                nums_1.remove(0 as usize);
            }
            times += 1
        }
        let l = ans.len();
        if flag == 2 {
            return  ((ans[l-1] + ans[l-2]) as f64)/2.0;
        } else {
            return ans[l-1] as f64;
        }
    }
}
