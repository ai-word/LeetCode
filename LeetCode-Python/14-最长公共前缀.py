class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if len(strs) == 0:
            return ""
        if len(strs) == 1:
            return strs[0]
        tmin = min(strs)
        tmax = max(strs)
        for i,c in enumerate(tmin):
            if c != tmax[i]:
                return tmin[:i]

        return tmin
