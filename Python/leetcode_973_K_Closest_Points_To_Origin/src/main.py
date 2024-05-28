import heapq
import math

class Solution:
    def kClosest(self, points, k):
        min_heap = []

        for (x, y) in points:
            distance = math.sqrt(x ** 2 + y ** 2 )
            heapq.heappush(min_heap, (distance, [x, y]))

        return [heapq.heappop(min_heap)[1] for _ in range(k)]

if __name__ == '__main__':
    points = [[1, 3], [-2, 2]]
    k = 1
    solution = Solution()
    print(solution.kClosest(points, k))
