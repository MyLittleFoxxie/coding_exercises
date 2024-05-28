import heapq

class Solution(object):
    def findKthLargest(self, nums, k):
        min_heap = []

        for num in nums:
            if len(min_heap) < k:
                heapq.heappush(min_heap, num)
            else:
                heapq.heappushpop(min_heap, num)

        return min_heap[0]

if __name__ == '__main__':
    nums = [3,2,3,1,2,4,5,5,6]
    k = 4
    solution = Solution()
    print(solution.findKthLargest(nums, k))