use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    println!("{:?}", Solution::kth_smallest_prime_fraction(vec![1,2,3,5], 3));
    println!("{:?}", Solution::kth_smallest_prime_fraction(vec![1,7], 1));
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

    // Link: https://leetcode.com/problems/k-th-smallest-prime-fraction
    pub fn kth_smallest_prime_fraction(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        arr.sort();


        #[derive(PartialEq, Eq)]
        struct Element {
            nume: i32,
            denom: i32
        }
        impl Element {
            fn new(nume: i32, denom: i32) -> Element {
                Element { nume, denom }
            }

            fn value(&self) -> f32 {
                self.nume as f32 / self.denom as f32
            }
        }
        impl PartialOrd for Element {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Element {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.value().total_cmp(&other.value())
            }
        }

        impl Into<Vec<i32>> for Element {
            fn into(self) -> Vec<i32> {
                vec![self.nume, self.denom]
            }
        }
        let mut max_heap = BinaryHeap::new();
        let arr_len = arr.len();
        for i in 0..arr_len {
            for j in (i+1..=arr_len-1).rev() {
                max_heap.push(Element::new(arr[i], arr[j]));
                if max_heap.len() > k as usize {
                    max_heap.pop();
                }
            }
        }
        max_heap.pop().unwrap().into()
    }
}

// Link: https://leetcode.com/problems/seat-reservation-manager/
struct SeatManager {
    min_q: BinaryHeap<Reverse<i32>>
}
impl SeatManager {

    fn new(n: i32) -> Self {
        let mut min_q = BinaryHeap::new();
        for i in 1..=n {
            min_q.push(Reverse(i))
        }
        SeatManager {
            min_q,
        }
    }
    
    fn reserve(&mut self) -> i32 {
        self.min_q.pop().unwrap().0
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.min_q.push(Reverse(seat_number))
    }
}