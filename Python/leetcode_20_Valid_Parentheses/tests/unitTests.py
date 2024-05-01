# python3 -m unittest tests/test1.py

import unittest
from src.main import valid_parenthesis

class UnitTests(unittest.TestCase):
    def test1(self):
        self.assertEqual(valid_parenthesis("()"), True)

    def test2(self):
        self.assertEqual(valid_parenthesis("()[]{}"), True)
    
    def test3(self):
        self.assertEqual(valid_parenthesis("(]"), False)

if __name__ == '__main__':
    unittest.main()
