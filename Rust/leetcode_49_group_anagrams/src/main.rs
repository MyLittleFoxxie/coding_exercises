// Given an array of strings strs, group the anagrams together. You can return the answer in any order.

// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

// Example 1:

// Input: strs = ["eat","tea","tan","ate","nat","bat"]
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
// Example 2:

// Input: strs = [""]
// Output: [[""]]
// Example 3:

// Input: strs = ["a"]
// Output: [["a"]]

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let input1: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .into_iter()
        .map(String::from)
        .collect();
    let input2: Vec<String> = vec![""].into_iter().map(String::from).collect();
    let input3: Vec<String> = vec!["a"].into_iter().map(String::from).collect();

    let group_anagrams1 = group_anagrams(input1);
    let group_anagrams2 = group_anagrams(input2);
    let group_anagrams3 = group_anagrams(input3);

    println!("{:?}", group_anagrams1);
    println!("{:?}", group_anagrams2);
    println!("{:?}", group_anagrams3);

    Ok(())
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut count_key = [0; 26];

        for char in s.chars() {
            let index = (char as u8) - b'a';
            count_key[index as usize] += 1;
        }

        result.entry(count_key).or_insert_with(Vec::new).push(s);
    }

    result.into_values().collect()
}
