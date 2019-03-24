/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function(x) {
     if (x < 0) {
        return false;
    }
    var sum = 0;
    var n = x;
    while (n != 0) {
        sum = sum *10 + n%10;
        n /= 10;
        n = Math.floor(n)
    }
    if (sum == x) {
        return true;
    }
    return false 
};
