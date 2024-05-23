# python3 src/main.py < inputs/input1.txt

# Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

# Implement the MinStack class:

# MinStack() initializes the stack object.
# void push(int val) pushes the element val onto the stack.
# void pop() removes the element on the top of the stack.
# int top() gets the top element of the stack.
# int getMin() retrieves the minimum element in the stack.
# You must implement a solution with O(1) time complexity for each function.

class MinStack(object):

    def __init__(self):
        """
        Initialize the stack object.
        """
        self.data = []

    def push(self, val):
        """
        Pushes the element val onto the stack.
        
        :type val: int
        :rtype: None
        """
        if self.data:
            # Append the value and the current minimum
            self.data.append([val, min(val, self.data[-1][1])])
        else:
            # Stack is empty, so the minimum is the value itself
            self.data.append([val, val])

    def pop(self):
        """
        Removes the element on the top of the stack.
        
        :rtype: None
        """
        if self.data:
            self.data.pop()
        else:
            raise IndexError("pop from empty stack")

    def top(self):
        """
        Gets the top element of the stack.
        
        :rtype: int
        """
        if self.data:
            return self.data[-1][0]
        else:
            raise IndexError("top from empty stack")

    def getMin(self):
        """
        Retrieves the minimum element in the stack.
        
        :rtype: int
        """
        if self.data:
            return self.data[-1][1]
        else:
            raise IndexError("getMin from empty stack")


# Example usage and testing
if __name__ == "__main__":
    obj = MinStack()
    obj.push(2)
    obj.push(1)
    obj.push(3)
    obj.pop()
    param_3 = obj.top()
    param_4 = obj.getMin()
    print("Final Stack data:", obj.data)
    print("Top element:", param_3)
    print("Minimum element:", param_4)

    # Additional tests
    obj.pop()
    print("Minimum element after popping:", obj.getMin())  # Should be 2
    obj.pop()
    try:
        obj.pop()  # This should raise an exception
    except IndexError as e:
        print("Caught an exception:", e)
