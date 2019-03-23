class Solution:
    def isPalindrome(self, x: int) -> bool:
        s = str(x)[::-1]
        return s == str(x)
        
