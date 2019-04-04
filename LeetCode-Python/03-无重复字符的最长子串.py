class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        sum_str = ""
        i = k = 0
        lens = len(s)
        max = 0
        while i < lens and k < lens:
            if sum_str.find(s[k]) == -1:
                sum_str += str(s[k])
                k += 1
            else:
                i += 1
                k = i
                sum_str = ""
            if max < len(sum_str):
                max = len(sum_str)

        return  max

