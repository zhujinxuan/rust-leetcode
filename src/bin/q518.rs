use std::vec::Vec;
use std::convert::TryFrom;

struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins : Vec<i32>) -> i32 {
        usize::try_from(amount)
            .ok()
            .map(|a| Self::nice(a, coins))
            .flatten()
            .unwrap_or(0)
    }

    fn nice(amount : usize, coins : Vec<i32>) -> Option<i32> {
        let mut res = vec![1];
        res.resize_with(amount + 1, || 0);
        for c in coins.iter() {
            Self::update_count(&mut res, &c);
        }
        res.last().map(|c| *c)
    }

    fn update_count(counts : & mut Vec<i32>, ic : &i32) {
        match usize::try_from(*ic).ok() {
            None => (),
            Some(c) => match counts.len().checked_sub(c) {
                None => (),
                Some(max_pos) => for pos in 0..max_pos {
                    counts[pos+c] += counts[pos];
                }
            }
        }
    }
}

fn main() {
    assert_eq!(1, Solution::change(10, vec![1]));
    assert_eq!(4, Solution::change(5, vec![1,2,5]));
}
