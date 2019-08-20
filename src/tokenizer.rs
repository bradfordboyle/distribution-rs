use std::collections::HashMap;
use std::io;

use pairlist::Pair;
use regex::Regex;

pub trait Tokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> Vec<Pair>;
}

pub struct PreTalliedTokenizer {
    re: Regex,
}

impl PreTalliedTokenizer {
    pub fn key_value_tokenizer() -> PreTalliedTokenizer {
        PreTalliedTokenizer {
            re: Regex::new(r"^\s*(?P<key>.+)\s+(?P<value>\d+)$").unwrap(),
        }
    }

    pub fn value_key_tokenizer() -> PreTalliedTokenizer {
        PreTalliedTokenizer {
            re: Regex::new(r"^\s*(?P<value>\d+)\s+(?P<key>.+)$").unwrap(),
        }
    }
}

impl Tokenizer for PreTalliedTokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> Vec<Pair> {
        let mut vec = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            // TODO stop unwrapping
            let caps = self.re.captures(line.as_str()).unwrap();
            let value = caps.name("value").unwrap().as_str().parse::<u64>().unwrap();
            let key = caps.name("key").unwrap().as_str();
            vec.push(Pair::new(value, key));
        }

        vec
    }
}

pub struct LineTokenizer {
    re: Regex,
}

impl LineTokenizer {
    pub fn new(matcher: &str) -> LineTokenizer {
        LineTokenizer {
            re: Regex::new(matcher).unwrap(),
        }
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

pub struct RegexTokenizer {
    splitter: Regex,
    matcher: Regex,
}

impl RegexTokenizer {
    pub fn new(splitter: &str, matcher: &str) -> RegexTokenizer {
        let splitter_re = match splitter {
            "white" => Regex::new(r"\s+").unwrap(),
            "word" => Regex::new(r"\W").unwrap(),
            _ => Regex::new(splitter).unwrap(),
        };

        let matcher_re = match matcher {
            "word" => Regex::new(r"^[A-Z,a-z]+$").unwrap(),
            "num" => Regex::new(r"^\d+$").unwrap(),
            _ => Regex::new(matcher).unwrap(),
        };

        RegexTokenizer {
            splitter: splitter_re,
            matcher: matcher_re,
        }
    }
}

impl Tokenizer for RegexTokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> Vec<Pair> {
        let mut counts: HashMap<String, u64> = HashMap::new();

        for l in reader.lines() {
            let line = l.unwrap();
            for token in self.splitter.split(line.trim_end()) {
                if self.matcher.is_match(token) {
                    let value = counts.entry(String::from(token)).or_insert(0);
                    *value += 1
                }
            }
        }

        let mut vec = Vec::new();
        for (key, &value) in &counts {
            vec.push(Pair::new(value, key))
        }
        debug!("[vec={:?}]", vec);
        vec
    }
}

#[cfg(test)]
mod test {
    use pairlist::Pair;
    use std::io;
    use tokenizer::{LineTokenizer, PreTalliedTokenizer, RegexTokenizer, Tokenizer};

    #[test]
    fn key_value_tokenize_empty_reader() {
        let t = PreTalliedTokenizer::key_value_tokenizer();
        let c = io::Cursor::new(b"");
        assert_eq!(t.tokenize(c), vec![]);
    }

    #[test]
    fn key_value_tokenize_single_line() {
        let t = PreTalliedTokenizer::key_value_tokenizer();
        let c = io::Cursor::new(b"a 1\n");
        assert_eq!(t.tokenize(c), vec![Pair::new(1, "a")]);
    }

    #[test]
    fn key_value_tokenize_multiple_lines() {
        let t = PreTalliedTokenizer::key_value_tokenizer();
        let c = io::Cursor::new(b"aa 1\nab 2\nba 1");
        assert_eq!(t.tokenize(c), vec![Pair::new(1, "aa"), Pair::new(2, "ab"), Pair::new(1, "ba")]);
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
        assert_eq!(t.tokenize(c), vec![Pair::new(1, "aa"), Pair::new(2, "ab"), Pair::new(1, "ba")]);
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
        assert_eq!(actual, vec![Pair::new(1, "2 ab"), Pair::new(1, "1 ba"), Pair::new(1, "1 aa")]);
    }

    #[test]
    fn regex_tokenizer() {
        let t = RegexTokenizer::new(r"/", r".+");
        let c = io::Cursor::new("/var/log/apparmor\n/var/log/dmesg.1.gz");
        let mut actual = t.tokenize(c);

        actual.sort_by(|a, b| b.cmp(&a));
        assert_eq!(
            actual,
            vec![
                Pair::new(2, "var"),
                Pair::new(2, "log"),
                Pair::new(1, "dmesg.1.gz"),
                Pair::new(1, "apparmor"),
            ]
        );
    }
}
