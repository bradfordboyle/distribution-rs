extern crate regex;

mod tokenizer;
mod pairlist;
mod histogram;
mod settings;

use std::env;
use std::io;

use histogram::HistogramWriter;
use tokenizer::{LineTokenizer, PreTalliedTokenizer, RegexTokenizer};
use tokenizer::Tokenizer;
use settings::{PreTallied, Settings};

fn main() {
    let s = Settings::new(env::args());
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut p = if s.graph_values() == &PreTallied::ValueKey {
        PreTalliedTokenizer::value_key_tokenizer().tokenize(stdin_lock)
    } else if s.graph_values() == &PreTallied::KeyValue {
        PreTalliedTokenizer::key_value_tokenizer().tokenize(stdin_lock)
    } else if s.tokenize() != "" {
        RegexTokenizer::new(s.tokenize(), s.match_regexp()).tokenize(stdin_lock)
    } else {
        LineTokenizer::new(r".").tokenize(stdin_lock)
    };

    p.sort_by(|a, b| b.cmp(a));
    let h = HistogramWriter::new(s);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    h.write_histogram(&mut handle, &p).expect("Unable to write histogram to STDOUT");
}
