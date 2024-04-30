# Given an integer array nums, return an array answer such that answer[i] 
# is equal to the product of all the elements of nums except nums[i].

# The product of any prefix or suffix of nums is guaranteed to fit in 
# a 32-bit integer.

# You must write an algorithm that runs in O(n) time and without using 
# the division operation.
class Solution(object):
    def productExceptSelf(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        result = []
        for i in range(0, len(nums)):
            total = 1
            for index, x in enumerate(nums):
                if i == index:
                    continue

                total *= x
            result.append(total)
        
        return result

solution = Solution()
input1 = [1,2,3,4]
input2 = [-1,1,0,-3,3]

print(solution.productExceptSelf(input1))
print(solution.productExceptSelf(input2))

