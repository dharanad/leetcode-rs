fn main() {
    println!("{}", Solution::compare_version("1.01.2".to_string(), "1.01.02".to_string(),));
    println!("{}", Solution::compare_version("1.1".to_string(), "1".to_string(),));
    println!("{}", Solution::compare_version("2.1".to_string(), "1".to_string(),));
    println!("{}", Solution::compare_version("0.1".to_string(), "1".to_string(),));
    println!("{}", Solution::compare_version("1.0.0.0".to_string(), "1.0".to_string(),));
}

struct Solution;


impl Solution {
    // Link: https://leetcode.com/problems/reverse-prefix-of-word/
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(idx) = Solution::find_index(&word, ch) {
            Solution::reverse(word, 0, idx)
        } else {
            word
        }
    }

    pub fn find_index(word: &str, ch: char) -> Option<usize> {
        for (idx, c) in word.chars().enumerate() {
            if c == ch {
                return Some(idx)
            }
        }
        None
    }

    pub fn reverse(s: String, from: usize, to: usize) -> String {
        let mut s = s;
        let chunk = &s[from..=to];
        let reversed_chunk = chunk.chars().rev().collect::<String>();
        s.replace_range(from..=to, &reversed_chunk);
        s
    }

    // Link: https://leetcode.com/problems/largest-positive-integer-that-exists-with-its-negative
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        use std::cmp::max;
        
        let mut s = HashSet::new();
        let mut res = -1;
        for n in nums {
            if s.contains(&-n) {
                res = max(res, n.abs())
            }
            s.insert(n);
        }
        res
    }

    // Link: https://leetcode.com/problems/compare-version-numbers
    pub fn compare_version(version1: String, version2: String) -> i32 {
        use std::cmp;
        let mut v1_chunks = version1.split(".").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut v2_chunks = version2.split(".").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        drop(version1);
        drop(version2);

        let diff = v1_chunks.len().abs_diff(v2_chunks.len());
        if v1_chunks.len() < v2_chunks.len() {
            v1_chunks.extend(vec![0; diff]);        
        } else {
            v2_chunks.extend(vec![0; diff]);
        }
        
        for (a, b) in v1_chunks.iter().zip(v2_chunks.iter()) {
            match a.cmp(b) {
                cmp::Ordering::Less => {
                    return -1
                },
                cmp::Ordering::Greater => {
                    return 1
                }
                cmp::Ordering::Equal => {
                    // Pass
                }
            }
        }
        0
    }
}


