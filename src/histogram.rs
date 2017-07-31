use std::cmp;
use std::io;

use pairlist::Pair;
use settings::Settings;
use std::io::Write;

pub struct HistogramWriter {
    s: Settings,
    height: usize,
    width: usize,
}

impl HistogramWriter {
    pub fn new(s: Settings) -> HistogramWriter {
        let w = s.width();
        let h = s.height();
        HistogramWriter { s: s, width: w, height: h}
    }

    pub fn write_histogram<T: io::Write>(&self, writer: &mut T, pairlist: &Vec<Pair>) {
        let output_limit = cmp::min(self.height, pairlist.len());
        let data: Vec<_> = pairlist.iter().take(output_limit).collect();
        let max_pct_width = 8usize;
        let regular_color = "\u{001b}[0m";
        let key_color = "\u{001b}[32m";
        let ct_color = "\u{001b}[34m";
        let pct_color = "\u{001b}[35m";
        let graph_color = "\u{001b}[37m";


        let total_value = pairlist.iter().fold(0, |sum, p| sum + p.value);
        let max_value = data.iter().fold(0, |max, p| cmp::max(max, p.value));

        let max_key_width = data.iter().fold(0, |max, p| cmp::max(max, p.key.len()));
        let max_token_width = format!("{}", max_value).len();

        let bar_width = self.width - (max_key_width+1) - (max_token_width+1) - (max_pct_width+1) - 1;

        let mut stderr = io::stderr();
        write!(stderr, "{:>width$}", "Key", width = max_key_width);
        write!(stderr, "|{:>width$}", "Ct", width = max_token_width);
        write!(stderr, " {:>width$}", "(Pct)", width = max_pct_width);
        write!(stderr, " Histogram\n");
        write!(stderr, "{}|{}\n", "-".repeat(max_key_width), "-".repeat(self.width - 4));


        for (i, p) in data.iter().enumerate() {
            let pct = p.value as f64 / total_value as f64 * 100.0f64;

            write!(writer, "{:>width$}", p.key, width = max_key_width);
            write!(writer, "{}", regular_color);
            write!(writer, "|");
            write!(writer, "{}", ct_color);
            write!(writer, "{:>width$}", p.value, width = max_token_width);
            write!(writer, " ");

            // A good way to ensure padding is applied is to format your input,
            // then use this resulting string to pad your output.
            // https://doc.rust-lang.org/std/fmt/
            write!(writer, "{}", pct_color);
            write!(writer, "{:>width$}", format!("({:2.2}%)", pct), width = max_pct_width);

            write!(writer, "{}", graph_color);
            write!(writer, " {}", self.histogram_bar(max_value, bar_width, p.value));

            if i == output_limit - 1 {
                write!(writer, "{}", regular_color);
            } else {
                write!(writer, "{}\n", key_color);
            }
        }
    }

    fn histogram_bar(&self, max_value: u64, bar_width: usize, bar_value: u64) -> String {
        let width = (bar_value as f64) / (max_value as f64) * (bar_width as f64);
        let int_width = width.floor() as usize;
        let rem = width - int_width as f64;
        let graph_char = vec!["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
        let char_width = 1.0f64;
        let zero_char = "•";
        let one_char = "•";

        let mut bar = zero_char.repeat(int_width);

        if char_width == 1.0f64 {
            bar.push_str(one_char);
        } else if char_width < 1.0f64 && rem > char_width {
            let which = (rem / char_width).floor() as usize;
            bar.push_str(graph_char[which])
        }

        bar
    }
}
#[cfg(test)]
mod test {
    use histogram::HistogramWriter;

    #[test]
    fn histogram_test() {
        let h = HistogramWriter::new();
        assert_eq!(1, 1);
    }
}
