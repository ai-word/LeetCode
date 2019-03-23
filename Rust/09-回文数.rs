impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut sum = 0;
        let mut n = x;
        while n != 0 {
            sum = sum *10 + n%10;
            n /= 10;
        }
        if sum == x {
            return true;
        }
        return false
    }
}
