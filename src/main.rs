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
use settings::Settings;



fn main() {
    let s = Settings::new(env::args());
    // println!("{:?}", s);
    let stdin = io::stdin();
    let mut p = if s.graph_values() == "vk" {
        PreTalliedTokenizer::value_key_tokenizer().tokenize(stdin.lock())
    } else if s.tokenize() != "" {
        RegexTokenizer::new(s.tokenize(), s.match_regexp()).tokenize(stdin.lock())
    } else {
        LineTokenizer::new(r".").tokenize(stdin.lock())
    };

    p.sort_by(|a, b| b.cmp(a));
    // println!("{:?}", p);
    let h = HistogramWriter::new(s);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    h.write_histogram(&mut handle, &p);
}
