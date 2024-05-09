use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn main() {
    println!("{}", Solution::num_rescue_boats(vec![3, 5, 3, 4], 5)); // 4
    println!("{}", Solution::num_rescue_boats(vec![1, 3, 3, 4], 5)); // 3
    println!("{}", Solution::num_rescue_boats(vec![1, 2], 3)); // 1
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
                return Some(idx);
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
        use std::cmp::max;
        use std::collections::HashSet;

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
        let mut v1_chunks = version1
            .split(".")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut v2_chunks = version2
            .split(".")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

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
                cmp::Ordering::Less => return -1,
                cmp::Ordering::Greater => return 1,
                cmp::Ordering::Equal => {
                    // Pass
                }
            }
        }
        0
    }

    // Link: https://leetcode.com/problems/boats-to-save-people/
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort(); // sorting the vector since can_rescue_all_with_boats expect vec to be sorted
        let mut lo = 0;
        let mut hi = people.len() as i32;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if Solution::can_rescue_all_with_boats(&people, limit, mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }

    pub fn can_rescue_all_with_boats(people: &[i32], limit: i32, max_boats: i32) -> bool {
        let mut boat_count = 0;
        let mut lo = 0 as usize;
        let mut hi = people.len() - 1;
        while lo < hi {
            let sum = people[lo] + people[hi];
            if sum <= limit {
                // Pair People
                lo += 1;
                hi -= 1;
            } else {
                hi -= 1;
            }
            boat_count += 1;
        }
        if lo == hi {
            // if there is person left, use a boat
            boat_count += 1;
        }
        boat_count <= max_boats
    }

    // Link: https://leetcode.com/problems/maximize-happiness-of-selected-children
    pub fn maximum_happiness_sum(happiness: Vec<i32>, mut k: i32) -> i64 {
        use std::collections::binary_heap::BinaryHeap;
        // We can also sort in desc order and stop when delta >= ele
        use std::ops::Sub;
        let mut delta = 0;
        let mut res = 0;
        let mut max_q: BinaryHeap<i32> = BinaryHeap::new();
        max_q.extend(happiness.iter());
        while let Some(ele) = max_q.pop() {
            if k == 0 {
                break;
            }
            res += ele.sub(delta).max(0) as i64;
            delta += 1;
            k -= 1;
        }
        res
    }

    // Link: https://leetcode.com/problems/relative-ranks
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        use std::collections::HashMap;
        let mut relative_score = score.clone();
        relative_score.sort_unstable_by_key(|x| -x);
        let mut score_rank_map = HashMap::new();
        for (idx, ele) in relative_score.iter().enumerate() {
            score_rank_map.insert(*ele, idx); // store score and its rank
        }
        score
            .iter()
            .map(|x| score_rank_map.get(x).unwrap().to_owned())
            .map(|r| Solution::stringfy_range(r))
            .collect()
    }

    pub fn stringfy_range(rank: usize) -> String {
        return match rank {
            0 => format!("Gold Medal"),
            1 => format!("Silver Medal"),
            2 => format!("Bronze Medal"),
            x => format!("{}", x + 1),
        };
    }
}
