use std::cmp;
use std::io;

use pairlist::Pair;

pub struct HistogramWriter {
    height: usize,
    width: usize,
}

impl HistogramWriter {
    pub fn new() -> HistogramWriter {
        HistogramWriter { width: 80, height: 15}
    }

    pub fn write_histogram<T: io::Write>(&self, writer: &mut T, pairlist: &Vec<Pair>) {
        let max_key_width = 3usize;
        let max_token_width = 5usize;
        let max_pct_width = 8usize;
        let regular_color = "\u{001b}[0m";
        let key_color = "\u{001b}[0m";
        let ct_color = "\u{001b}[32m";
        let pct_color = "\u{001b}[35m";
        let graph_color = "\u{001b}[34m";


        let total_value = pairlist.iter().fold(0, |sum, p| sum + p.value);
        let max_value = pairlist.iter().fold(0, |max, p| cmp::max(max, p.value));

        let bar_width = self.width - (max_key_width+1) - (max_token_width+1) - (max_pct_width+1);

        write!(writer, "{:>width$}", "Key", width = max_key_width);
        write!(writer, "|{:>width$}", "Ct", width = max_token_width);
        write!(writer, " {:>width$}", "(Pct)", width = max_pct_width);
        write!(writer, " Histogram\n");
        write!(writer, "{}|{}\n", "-".repeat(max_key_width), "-".repeat(self.width - 4));

        let output_limit = 15usize;
        for p in pairlist.iter().take(output_limit) {
            let pct = p.value as f64 / total_value as f64 * 100.0f64;

            write!(writer, "{}", key_color);
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
            write!(writer, " {}\n", self.histogram_bar(max_value, bar_width, p.value));
            write!(writer, "{}", regular_color);
        }
    }

    fn histogram_bar(&self, max_value: u64, bar_width: usize, bar_value: u64) -> String {
        let width = (bar_value as f64) / (max_value as f64) * (bar_width as f64);
        let int_width = width.floor() as usize;
        let rem = width - int_width as f64;
        let graph_char = vec!["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
        let char_width = 0.125f64;
        let zero_char = "█";

        let mut bar = zero_char.repeat(int_width);

        if rem > char_width {
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
