use std::io;
use regex::Regex;

trait Tokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> u64;
}

struct PreTalliedTokenizer;

impl Tokenizer for PreTalliedTokenizer {
    fn tokenize<T: io::BufRead>(&self, reader: T) -> u64 {
        // TODO move into self
        let re = Regex::new(r"\s*(.+)\s+(\d+)").unwrap();

        let mut sum = 0u64;
        for line in reader.lines() {
            let foo = line.unwrap();
            // TODO stop unwrapping
            let caps = re.captures(foo.as_str()).unwrap();
            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            sum += value
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use tokenizer::PreTalliedTokenizer;
    use tokenizer::Tokenizer;

    #[test]
    fn tokenize_empty_reader() {
        let t = PreTalliedTokenizer;
        let c = io::Cursor::new(b"");
        assert_eq!(t.tokenize(c), 0);
    }

    #[test]
    fn tokenize_single_line() {
        let t = PreTalliedTokenizer;
        let c = io::Cursor::new(b"a 1\n");
        assert_eq!(t.tokenize(c), 1);
    }
}
