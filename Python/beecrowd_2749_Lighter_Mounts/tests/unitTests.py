# python3 -m unittest tests/test1.py

import unittest
from src.main import lighter_mounts

class UnitTests(unittest.TestCase):
    def test1(self):
        self.assertEqual(lighter_mounts(""), True)

if __name__ == '__main__':
    unittest.main()
