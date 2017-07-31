extern crate regex;

mod tokenizer;
mod pairlist;
mod histogram;
mod settings;

use std::env;
use std::io;

use histogram::HistogramWriter;
use tokenizer::LineTokenizer;
use tokenizer::Tokenizer;
use settings::Settings;



fn main() {
    let s = Settings::new(env::args());
    let t = LineTokenizer::new(r".");
    let stdin = io::stdin();

    let mut p = t.tokenize(stdin.lock());

    p.sort_by(|a, b| b.cmp(a));
    let h = HistogramWriter::new(s);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    h.write_histogram(&mut handle, &p);
}

