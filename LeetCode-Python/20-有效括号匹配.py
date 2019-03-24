class Solution:
    def isValid(self, s):
        stack = list(['1'])
        for st in s:
            if st in ['(', '[', '{']:
                stack.append(st)
            elif (stack[len(stack)-1]+st) not in ['()', '[]', '{}']:
                stack.append(st)
            else:
                stack.pop()
        if stack == ['1']:
            return True
        else:
            return False
