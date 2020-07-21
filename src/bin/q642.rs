use std::collections::HashMap;
use std::str::Chars;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct AutocompleteSystem {
    trie: Option<Rc<RefCell<Trie>>>,
    orig : Option<Rc<RefCell<Trie>>>,
    sentence: String,
}

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Rc<RefCell<Trie>>>,
    time: Option<Rc<RefCell<PString>>>
}

impl Trie {
    fn empty() -> Self {
        return Trie {
            children: HashMap::new(),
            time: None,
        }
    }

    fn add_sentence(&mut self, mut chars: Chars, t: PString) {
        match chars.next() {
            None => {
                match self.time.take() {
                    None => {self.time.replace(Rc::new(RefCell::new(t)));}
                    Some(x) => {
                        x.borrow_mut().0 += t.0;
                        self.time.replace(x.clone());
                    }
                };
            }
            Some(c) => {
                let child = self.children.entry(c).or_insert(
                    Rc::new(RefCell::new(Trie::empty()))
                );
                child.borrow_mut().add_sentence(chars, t)
            }
        }
    }

    fn new(sentences: Vec<String>, times : Vec<i32>) -> Self {
        let mut res = Self::empty();
        for (s, &t) in sentences.iter().zip(times.iter()) {
            res.add_sentence(s.chars(), PString(t, s.clone()));
        }
        res
    }

    fn auto(&self) -> Vec<Rc<RefCell<PString>>> {
        let mut res = Vec::with_capacity(6);
        self.time.as_ref().map(|v| res.push(v.clone()));
        for (_,v) in self.children.iter() {
            for p in v.borrow().auto().iter() {
                res.push(p.clone());
            }
            res.sort_by(|ra,rb| {
                let a = ra.borrow();
                let b = rb.borrow();
                match a.0.cmp(&b.0) {
                    Ordering::Equal => a.1.cmp(& b.1),
                    Ordering::Greater => Ordering::Less,
                    Ordering::Less => Ordering::Greater,
                }
            });
            while res.len() > 3 {
                res.pop();
            }
        }
        res
    }

}

#[derive(Debug)]
struct PString(i32, String);


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AutocompleteSystem {
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let t = Trie::new(sentences, times);
        let trie = Some(Rc::new(RefCell::new(t)));
        AutocompleteSystem {
            trie: trie.as_ref().map(|v| v.clone()),
            orig: trie.as_ref().map(|v| v.clone()),
            sentence: String::from(""),
        }
    }

    fn input(& mut self, c: char) -> Vec<String> {
        if c == '#' {
            let s = self.sentence.clone();
            self.sentence = String::from("");
            self.trie = self.orig.as_ref().map(|v| v.clone());
            self.orig.as_ref().map(|v| {
                v.borrow_mut().add_sentence(s.chars(), PString(1,s.clone()));
            });
            return Vec::new();
        }

        self.sentence.push(c);
        match self.trie.take() {
            None => Vec::new(),
            Some(trie) => {

                trie.borrow().children.get(&c).map_or(
                    Vec::new(),
                    |v| {
                        self.trie.replace(v.clone());
                        let mut res = Vec::with_capacity(3);
                        for e in v.borrow().auto().iter() {
                            res.push(e.borrow().1.clone());
                        }
                        res
                    })
            }
        }
    }
}

/**
 * Your AutocompleteSystem object will be instantiated and called as such:
 * let obj = AutocompleteSystem::new(sentences, times);
 * let ret_1: Vec<String> = obj.input(c);
 */
fn main() {
    let mut amp = AutocompleteSystem::new(
        vec![
            String::from("i love you"),
            String::from("island"),
            String::from("ironman"),
            String::from("i love leetcode")],
        vec![5,3,2,2]
    );

    // assert_eq!(Vec::<String>::new(), amp.input('#'));
    assert_eq!(Vec::<String>::new(), amp.input('i'));

}
