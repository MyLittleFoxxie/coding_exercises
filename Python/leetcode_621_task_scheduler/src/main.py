# You are given an array of CPU tasks, each represented by letters A to Z, 
# and a cooling time, n. Each cycle or interval allows the completion of 
# one task. Tasks can be completed in any order, 
# but there's a constraint: identical tasks must be separated by at least 
# n intervals due to cooling time.

# ​Return the minimum number of intervals required to complete all tasks.

# Example 1:

# Input: tasks = ["A","A","A","B","B","B"], n = 2

# Output: 8

# Explanation: A possible sequence is: A -> B -> idle -> A -> B -> idle -> A -> B.

import heapq as h
import collections as c

class Solution(object):
    def leastInterval(self, tasks, n):
        frequencyMap = {}
        for task in tasks:
            frequencyMap[task] = frequencyMap.get(task, 0) + 1
        
        frequencies = []
        for freq in frequencyMap.values():
            h.heappush(frequencies, -freq)

        taskQueue = c.deque()
        time = 0

        while frequencies or taskQueue:
            time += 1

            if frequencies:
                count = h.heappop(frequencies) + 1
                if count:
                    taskQueue.append([count, time + n])

            if taskQueue and taskQueue[0][1] == time:
                h.heappush(frequencies, taskQueue.popleft()[0])

        return time

if __name__ == "__main__":
    tasks = ["A","A","A","B","B","B"]
    n = 2
    solution = Solution()
    print(solution.leastInterval(tasks, n))