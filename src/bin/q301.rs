use std::str::Chars;
use std::mem;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let parens = Parens::from_chars(s.chars());
        parens.remove_invalid()
    }
}

#[derive(Debug)]
struct Parens {
    lefts: Vec<u32>,
    rights: Vec<u32>,
}

impl Parens {
    fn from_chars(s : Chars) -> Self {
        unimplemented!();
    }

    fn mismatch(&self) -> (Vec<u32>, Vec<u32>) {
        unimplemented!();
    }

    fn take(&self, n : usize) -> Self {
        unimplemented!();
    }

    fn last(&self, n : usize) -> Self {
        unimplemented!();
    }

    fn reverse(&mut self) {
        unimplemented!();
    }

    fn strip_both(&self, h : usize, t: usize) -> Self {
        unimplemented!();
    }

    fn remove_invalid(&self) -> Vec<String> {
        let (head_m, tail_m) = self.mismatch();
        let head_p = self.take(head_m.len());
        let tail_p = self.last(tail_m.len());
        let mid_p = self.strip_both(head_m.len(), tail_m.len());
        let head = head_p.remove_by_mismatch(head_m);
        let mut tail = tail_p.remove_by_mismatch(tail_m);

        for x in tail.iter_mut() {
            x.reverse()
        }

        let hs = head.iter().map(|x| x.build_string()).collect::<Vec<_>>();
        let ts = tail.iter().map(|x| x.build_string()).collect::<Vec<_>>();
        let mid = mid_p.build_string();
        let mut res = Vec::<String>::new();
        for h in hs.iter() {
            for t in ts.iter() {
                res.push(format!("{}{}{}", h, mid, t));
            }
        }
        res
    }

    fn remove_by_mismatch(&self, mismatch: Vec<u32>) -> Vec<Self> {
        unimplemented!();
    }

    fn build_string(&self) -> String {
        let mut res = String::new();
        for (l, r) in self.lefts.iter().zip(self.rights.iter()) {
            for _ in 0..(*l)  {
                res.push('(');
            }
            for _ in 0..(*r) {
                res.push(')');
            }
        }
        res
    }

}

fn main () {
    assert_eq!(Vec::<String>::new(), Solution::remove_invalid_parentheses(String::from(")(")));
}

struct Solution;
