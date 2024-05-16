use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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

    // Link: https://leetcode.com/problems/largest-local-values-in-a-matrix/
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let n = grid.len();
        for i in 0..n-2 {
            let mut row = Vec::new();
            for j in 0..n-2 {
                let local_max = grid[i][j].max(grid[i][j+1]).max(grid[i][j+2])
                    .max(grid[i+1][j]).max(grid[i+1][j+1]).max(grid[i+1][j+2])
                    .max(grid[i+2][j]).max(grid[i+2][j+1]).max(grid[i+2][j+2]);
                row.push(local_max);
            }
            res.push(row);
        }
        res
    }

    // Link: https://leetcode.com/problems/permutation-difference-between-two-strings/
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut char_index_map = HashMap::new();
        for (idx, c) in s.chars().enumerate() {
            char_index_map.insert(c, idx);
        }
        let mut res = 0;
        for (idx, c) in t.chars().enumerate() {
            res += idx.abs_diff(char_index_map.get(&c).unwrap().to_owned())
        }
        res as i32
    }
    
    // Link: https://leetcode.com/problems/find-k-closest-elements/
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        let mut max_q: BinaryHeap<(u32, i32)> = BinaryHeap::new();
        for e in arr.iter() {
            max_q.push((x.abs_diff(*e), *e));
            if max_q.len() > k as usize {
                max_q.pop();
            }
        }
        let mut aux =  max_q.into_vec().iter().map(|(_,b)| *b).collect::<Vec<i32>>();
        aux.sort();
        aux
    }

    // Link: https://leetcode.com/problems/unique-binary-search-trees/
    pub fn num_trees(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        let mut res = 0;
        for i in 0..n {
            res += Self::num_trees(i) * Self::num_trees(n - 1 - i);
        }
        res
    }

    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        for i in 0..n {
            // If first bit is zero
            if grid[i][0] == 0 {
                // Flip Row
                for j in 0..m {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        for j in 0..m {
            let mut zero_count = 0;
            for i in 0..n {
                if grid[i][j] == 0 {
                    zero_count += 1;
                }
            }
            // If there are more zero than ones
            if zero_count > n - zero_count {
                // Flip Column
                for i in 0..n {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        let mut score = 0;
        for i in 0..n {
            let mut row_score = 0;
            for j in 0..m {
                row_score += grid[i][j] << (m - j - 1);
            }
            score += row_score;
        }
        
        score  
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut ele_index_map: HashMap<i32, usize> = HashMap::new();
        for (idx, ele) in nums.iter().enumerate() {
            if let Some(other_idx) = ele_index_map.get(&(target - ele)) {
                return vec![other_idx.to_owned() as i32, idx as i32]
            }
            ele_index_map.insert(*ele, idx);
        }
        vec![]
    }

    // Link: https://leetcode.com/problems/evaluate-boolean-binary-tree
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val == 0 {
                return false;
            }
            if node.val == 1 {
                return true
            }
            let left = Self::evaluate_tree(node.left.clone());
            let right = Self::evaluate_tree(node.right.clone());
            return (node.val == 2 && (left || right)) || (left && right);
        }
        false
    }

    // Link: https://leetcode.com/problems/merge-two-binary-trees/
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return match (root1, root2) {
            (Some(one), Some(two)) => {
                let one = one.borrow();
                let two = two.borrow();
                let sum = one.val + two.val;
                let mut node = TreeNode::new(sum);
                node.left = Self::merge_trees(one.left.clone(), two.left.clone());
                node.right = Self::merge_trees(one.right.clone(), two.right.clone());
                Some(Rc::new(RefCell::new(node)))
            },
            (Some(one), None) => {
                Some(one)
            },
            (None, Some(two)) => {
                Some(two)
            },
            (None, None) => {
                None
            }
        }
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