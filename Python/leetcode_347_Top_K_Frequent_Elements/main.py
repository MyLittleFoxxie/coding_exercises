# Given an integer array nums and an integer k, 
# return the k most frequent elements. You may 
# return the answer in any order.

# Example 1:
# Input: nums = [1,1,1,2,2,3], k = 2
# Output: [1,2]

# Example 2:
# Input: nums = [1], k = 1
# Output: [1]

class Solution:
    def topKFrequent(self, nums, k):
        count = {}
        freq = [[] for i in range(len(nums) + 1)]

        for n in nums:
            count[n] = 1 + count.get(n, 0)
        for n, c in count.items():
            freq[c].append(n)

        res = []
        for i in range(len(freq) - 1, 0, -1):
            for n in freq[i]:
                res.append(n)
                if len(res) == k:
                    return res

# Test cases
solution = Solution()
input1 = [1, 1, 1, 2, 2, 3]
k1 = 2
input2 = [1]
k2 = 1
input3 = [1,5,5,9,9,9]
k3 = 3

print(solution.topKFrequent(input1, k1))  # Expected Output: [1, 2]
print(solution.topKFrequent(input2, k2))  # Expected Output: [1]
print(solution.topKFrequent(input3, k3))  # Expected Output: [1]