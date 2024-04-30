# python3 -m unittest tests/test1.py

import unittest
from src.main import longest_consecutive_sequence

class TestFactorial(unittest.TestCase):
    def test_factorial_positive(self):
        self.assertEqual(longest_consecutive_sequence([100,4,200,1,3,2]), 4)

    def test_factorial_zero(self):
        self.assertEqual(longest_consecutive_sequence([0,3,7,2,5,8,4,6,0,1]), 9)

if __name__ == '__main__':
    unittest.main()
