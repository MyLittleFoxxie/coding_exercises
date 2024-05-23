# Design a class to find the kth largest element in a stream. 
# Note that it is the kth largest element in the sorted order, 
# not the kth distinct element.
import heapq

class KthLargest(object):

    def __init__(self, k, nums):
        self.k = k
        self.nums = nums
        
        heapq.heapify(nums)
        while len(self.nums) > k:
            heapq.heappop(self.nums)

    def add(self, n):
        if len(self.nums) < self.k:
            heapq.heappush(self.nums, n)
        elif n > self.nums[0]:
            heapq.heappushpop(self.nums, n)
        
        return self.nums[0]
    

if __name__ == "__main__":
    kthLargest = KthLargest(3, [4, 5, 8, 2])
    print(kthLargest.add(3))
    print(kthLargest.add(5))
    print(kthLargest.add(10))
    print(kthLargest.add(9))
    print(kthLargest.add(4))
