import heapq as hp

class Solution(object):

    def __init__(self):
        pass
        
    def lastStoneWeight(self, stones):
        inverted_stones = [-stone for stone in stones]
        hp.heapify(inverted_stones)

        while len(inverted_stones) > 1:
            x = hp.heappop(inverted_stones)
            y = hp.heappop(inverted_stones)

            if x == y:
                continue

            z = x - y
            hp.heappush(inverted_stones, z)
        
        return inverted_stones[0] * -1 if inverted_stones else 0


if __name__ == "__main__":
    stones = [2,2]
    test = Solution()
    print(test.lastStoneWeight(stones))