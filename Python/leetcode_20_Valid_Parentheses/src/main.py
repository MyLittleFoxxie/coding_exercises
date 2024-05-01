# python3 src/main.py

# Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', 
# determine if the input string is valid.

# An input string is valid if:
# Open brackets must be closed by the same type of brackets.
# Open brackets must be closed in the correct order.
# Every close bracket has a corresponding open bracket of the same type.

# Example 1:
# Input: s = "()"
# Output: true

# Example 2:
# Input: s = "()[]{}"
# Output: true

# Example 3:
# Input: s = "(]"
# Output: false

def valid_parenthesis(s) -> bool:
    stack = []
    bracket_map = {')': '(', ']': '[', '}': '{'}

    for char in s:
        if char in bracket_map.values(): 
            stack.append(char)
        elif char in bracket_map.keys(): 
            if stack and stack[-1] == bracket_map[char]: 
                stack.pop()
            else:
                return False
        else:
            return False 

    return not stack  

if __name__ == "__main__":
    test1 = "()"
    test2 = "()[]{}"
    test3 = "(]"
    print(f"Test result: {valid_parenthesis(test1)}")
    print(f"Test result: {valid_parenthesis(test2)}")
    print(f"Test result: {valid_parenthesis(test3)}") 
