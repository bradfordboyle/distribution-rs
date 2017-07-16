use std::collections::HashMap;
use std::io;

use regex::Regex;
use pairlist::Pair;


pub trait Tokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> Vec<Pair>;
}

pub struct PreTalliedTokenizer {
    re: Regex
}

impl PreTalliedTokenizer {
    pub fn key_value_tokenizer() -> PreTalliedTokenizer {
        PreTalliedTokenizer { re: Regex::new(r"^\s*(?P<key>.+)\s+(?P<value>\d+)$").unwrap() }
    }

    pub fn value_key_tokenizer() -> PreTalliedTokenizer {
        PreTalliedTokenizer { re: Regex::new(r"^\s*(?P<value>\d+)\s+(?P<key>.+)$").unwrap() }
    }
}

impl Tokenizer for PreTalliedTokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> Vec<Pair> {
        let mut vec = Vec::new();
        for line in reader.lines() {
            let foo = line.unwrap();
            // TODO stop unwrapping
            let caps = self.re.captures(foo.as_str()).unwrap();
            let value = caps.name("value").unwrap().as_str().parse::<u64>().unwrap();
            let key = caps.name("key").unwrap().as_str();
            vec.push(Pair::new(value, key));
        }

        vec
    }
}

pub struct LineTokenizer {
    re: Regex
}

impl LineTokenizer {
    pub fn new(matcher: &str) -> LineTokenizer {
        LineTokenizer { re: Regex::new(matcher).unwrap() }
    }
}

impl Tokenizer for LineTokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> Vec<Pair> {
        let mut counts: HashMap<String, u64> = HashMap::new();

        for line in reader.lines() {
            let key = line.unwrap();
            if self.re.is_match(key.as_str()) {
                let value = counts.entry(key).or_insert(0);
                *value += 1
            }
        }

        let mut vec = Vec::new();
        for (key, &value) in &counts {
            vec.push(Pair::new(value, key))
        }
        vec
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use tokenizer::PreTalliedTokenizer;
    use tokenizer::LineTokenizer;
    use tokenizer::Tokenizer;
    use pairlist::Pair;

    #[test]
    fn key_value_tokenize_empty_reader() {
        let t = PreTalliedTokenizer::key_value_tokenizer();
        let c = io::Cursor::new(b"");
        assert_eq!(t.tokenize(c), vec![]);
    }

    #[test]
    fn key_value_tokenize_single_line() {
        let t = PreTalliedTokenizer::key_value_tokenizer();;
        let c = io::Cursor::new(b"a 1\n");
        assert_eq!(t.tokenize(c), vec![Pair::new(1, "a")]);
    }

    #[test]
    fn key_value_tokenize_multiple_lines() {
        let t = PreTalliedTokenizer::key_value_tokenizer();;
        let c = io::Cursor::new(b"aa 1\nab 2\nba 1");
        assert_eq!(t.tokenize(c), vec![
            Pair::new(1, "aa"),
            Pair::new(2, "ab"),
            Pair::new(1, "ba")
        ]);
    }

    #[test]
    fn value_key_tokenize_empty_reader() {
        let t = PreTalliedTokenizer::value_key_tokenizer();
        let c = io::Cursor::new(b"");
        assert_eq!(t.tokenize(c), vec![]);
    }

    #[test]
    fn value_key_tokenize_single_line() {
        let t = PreTalliedTokenizer::value_key_tokenizer();
        let c = io::Cursor::new(b"1 a\n");
        assert_eq!(t.tokenize(c), vec![Pair::new(1, "a")]);
    }

    #[test]
    fn value_key_tokenize_multiple_lines() {
        let t = PreTalliedTokenizer::value_key_tokenizer();
        let c = io::Cursor::new(b"1 aa\n2 ab\n1 ba");
        assert_eq!(t.tokenize(c), vec![
            Pair::new(1, "aa"),
            Pair::new(2, "ab"),
            Pair::new(1, "ba")
        ]);
    }

    #[test]
    fn line_tokenize_empty_reader() {
        let t = LineTokenizer::new(r".");
        let c = io::Cursor::new(b"");
        assert_eq!(t.tokenize(c), vec![]);
    }

    #[test]
    fn line_tokenize_single_line() {
        let t = LineTokenizer::new(r".");
        let c = io::Cursor::new(b"1 a\n");
        assert_eq!(t.tokenize(c), vec![Pair::new(1, "1 a")]);
    }

    #[test]
    fn line_tokenize_multiple_lines() {
        let t = LineTokenizer::new(r".");
        let c = io::Cursor::new(b"1 aa\n2 ab\n1 ba");
        let mut actual = t.tokenize(c);

        actual.sort_by(|a, b| b.cmp(&a));
        assert_eq!(actual, vec![
            Pair::new(1, "2 ab"),
            Pair::new(1, "1 ba"),
            Pair::new(1, "1 aa")
        ]);
    }
}
