# python3 src/main.py

# Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
# You must write an algorithm that runs in O(n) time.

# Example 1:
# Input: nums = [100,4,200,1,3,2]
# Output: 4
# Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

# Example 2:
# Input: nums = [0,3,7,2,5,8,4,6,0,1]
# Output: 9

def longest_consecutive_sequence(nums : list):
    nums_set = set(nums)
    largest_len = 0

    for n in nums:
        if n - 1 not in nums_set:
            len = 1
            while(n + 1 in nums_set):
                n += 1
                len += 1
            largest_len = max(largest_len, len)

    return largest_len

if __name__ == "__main__":
    test1 = [100,4,200,1,3,2]
    print(f"Test result: {longest_consecutive_sequence(test1)}")
