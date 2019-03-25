class Solution:
    def countAndSay(self, n: int) -> str:
        if n == 1:
            return "1"
        else:
            st = self.countAndSay(n-1)
            count = 1
            res = ""
            for i in range(0, len(st)):
                if i+1 < len(st) and st[i] == st[i+1]:
                    count += 1
                else:
                    res = res+ str(count)+str(st[i])
                    count = 1 
        return res