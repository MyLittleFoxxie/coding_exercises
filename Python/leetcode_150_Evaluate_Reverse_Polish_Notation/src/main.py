class Solution(object):
    import operator

    ops = {
        "+" : operator.add,
        "-" : operator.sub,
        "*" : operator.mul,
        "/" : operator.mod,
        # "/" : operator.truediv,
        # "%" : operator.mod,
        "^" : operator.xor,
    }

    def evalRPN(self, tokens):

        return
    
if __name__ == "__main__":
    tokens = ["2","1","+","3","*"]
    solution = Solution()
    print(solution.evalRPN(tokens))