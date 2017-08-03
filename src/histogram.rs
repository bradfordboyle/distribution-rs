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
        HistogramWriter {
            s: s,
            width: w,
            height: h,
        }
    }

    pub fn write_histogram<T: io::Write>(&self, writer: &mut T, pairlist: &Vec<Pair>) {
        let output_limit = cmp::min(self.height, pairlist.len());
        let data: Vec<_> = pairlist.iter().take(output_limit).collect();
        let max_pct_width = 8usize;

        let total_value = pairlist.iter().fold(0, |sum, p| sum + p.value);
        let max_value = data.iter().fold(0, |max, p| cmp::max(max, p.value));

        let max_key_width = data.iter().fold(0, |max, p| cmp::max(max, p.key.len()));
        let max_token_width = format!("{}", max_value).len();

        let bar_width = self.width - (max_key_width + 1) - (max_token_width + 1) -
                        (max_pct_width + 1) - 1;

        let mut stderr = io::stderr();
        write!(stderr, "{:>width$}", "Key", width = max_key_width);
        write!(stderr, "|{:>width$}", "Ct", width = max_token_width);
        write!(stderr, " {:>width$}", "(Pct)", width = max_pct_width);
        write!(stderr, " Histogram\n");
        write!(stderr,
               "{}|{}\n",
               "-".repeat(max_key_width),
               "-".repeat(self.width - 4));


        for (i, p) in data.iter().enumerate() {
            let pct = p.value as f64 / total_value as f64 * 100.0f64;

            write!(writer, "{:>width$}", p.key, width = max_key_width);
            write!(writer, "{}", self.s.regular_colour());
            write!(writer, "|");
            write!(writer, "{}", self.s.ct_colour());
            write!(writer, "{:>width$}", p.value, width = max_token_width);
            write!(writer, " ");

            // A good way to ensure padding is applied is to format your input,
            // then use this resulting string to pad your output.
            // https://doc.rust-lang.org/std/fmt/
            write!(writer, "{}", self.s.pct_colour());
            write!(writer,
                   "{:>width$}",
                   format!("({:2.2}%)", pct),
                   width = max_pct_width);

            write!(writer, "{}", self.s.graph_colour());
            write!(writer,
                   " {}",
                   self.histogram_bar(max_value, bar_width, p.value));

            if i == output_limit - 1 {
                write!(writer, "{}", self.s.regular_colour());
            } else {
                write!(writer, "{}\n", self.s.key_colour());
            }
        }
    }

    fn histogram_bar(&self, max_value: u64, bar_width: usize, bar_value: u64) -> String {
        let zero_char: char;
        let one_char: char;
        let histogram_char = self.s.histogram_char();
        if self.s.char_width() < 1f32 {
            zero_char = self.s
                .graph_chars()
                .last()
                .expect("graph_chars is empty")
                .clone();
            one_char = '\0'
        } else if histogram_char.len() > 1 && self.s.unicode_mode() == false {
            zero_char = histogram_char.chars()
                .nth(0)
                .unwrap()
                .clone();
            one_char = histogram_char.chars()
                .nth(1)
                .unwrap()
                .clone();
        } else {
            zero_char = histogram_char.chars()
                .nth(0)
                .expect("histogram_char is empty")
                .clone();
            one_char = zero_char;
        }

        let width = (bar_value as f64) / (max_value as f64) * (bar_width as f64);
        let int_width = width.floor() as usize;
        let rem = width - int_width as f64;
        let graph_char = vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        let char_width = 1.0f64;

        let mut bar = zero_char.to_string().repeat(int_width);

        if char_width == 1.0f64 {
            bar.push(one_char.clone());
        } else if char_width < 1.0f64 && rem > char_width {
            let which = (rem / char_width).floor() as usize;
            bar.push(graph_char[which])
        }

        bar
    }
}
#[cfg(test)]
mod test {
    use histogram::HistogramWriter;

    // #[test]
    // fn histogram_test() {
    //     let h = HistogramWriter::new();
    //     assert_eq!(1, 1);
    // }
}
