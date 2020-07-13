impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut  stream = Stream {s : &s};
        stream.consume()
    }
}

struct Stream<'a> {
    s : &'a str,
}

impl<'a> Stream<'a> {
    fn skip(&mut self, char : char) -> Option<()> {
        if Some(char) == self.s.chars().next() {
            self.s = self.s.get(1..).unwrap_or(self.s);
            Some(())
        } else {
            None
        }
    }

    fn take_while<F>(&mut self, f: F) -> Option<&str>
    where F: Fn(char) -> bool, {
        let mut res = 0;
        for c in self.s.chars() {
            if ! f(c) {
                break;
            }
            res +=1;
        }
        if res == 0 {
            None
        } else {
            let (f, s) = self.s.split_at(res);
            self.s = s;
            Some(f)
        }
    }

    fn numeric(&mut self) -> Option<usize> {
        self.take_while(char::is_numeric).map(
            |x| x.parse().ok()
        ).flatten()
    }

    fn normal_string(&mut self) -> Option<String> {
        self.take_while(|c| {
            return !char::is_numeric(c) && !"[]".contains(c)
        }).map(String::from)
    }

    fn consume(&mut self) -> String {
        let mut res = String::from("");
        loop {
            match self.normal_string().or_else(|| self.numeric_string()) {
                Some(v) => {res.push_str(&v);}
                None => {return res;}
            }
        }
    }

    fn numeric_string(&mut self) -> Option<String> {
        self.numeric()
            .map(|n| {
                self.skip('[');
                let content = self.consume();
                self.skip(']');
                content.repeat(n)
            })
    }
}


struct Solution {}
fn main() {
    assert_eq!("abc", Solution::decode_string(String::from("abc")));
}
