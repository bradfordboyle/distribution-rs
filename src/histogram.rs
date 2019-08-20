use std::cmp;
use std::io;

use pairlist::Pair;
use settings::Settings;

pub struct HistogramWriter {
    settings: Settings,
    height: usize,
    width: usize,
}

#[derive(Debug)]
struct ColumnWidths {
    key: usize,
    token: usize,
    pct: usize,
}

impl HistogramWriter {
    pub fn new(settings: Settings) -> HistogramWriter {
        let w = settings.width();
        let h = settings.height();
        HistogramWriter {
            settings,
            width: w,
            height: h,
        }
    }

    fn write_header<W: io::Write>(&self, w: &mut W, col_widths: ColumnWidths) -> io::Result<()> {
        write!(w, "{:>width$}", "Key", width = col_widths.key)?;
        write!(w, "|{:>width$}", "Ct", width = col_widths.token)?;
        write!(w, " {:>width$}", "(Pct)", width = col_widths.pct)?;
        writeln!(w, " Histogram")?;
        writeln!(
            w,
            "{}|{}",
            "-".repeat(col_widths.key),
            "-".repeat(self.width.checked_sub(col_widths.key + 1).unwrap_or(0))
        )?;

        Ok(())
    }

    fn pct_width(max: u64, total: u64) -> usize {
        format!("({:2.2}%)", 100.0 * max as f64 / total as f64).len()
    }

    pub fn write_histogram<T: io::Write>(&self, writer: &mut T, pairlist: &mut Vec<Pair>) -> io::Result<()> {
        let output_limit = cmp::min(self.height, pairlist.len());
        pairlist.sort_by(|a, b| b.cmp(a));
        let data: Vec<_> = pairlist.iter().take(output_limit).collect();

        let total_value = pairlist.iter().fold(0, |sum, p| sum + p.value());
        let max_value = data.iter().fold(0, |max, p| cmp::max(max, p.value()));
        let max_pct_width = HistogramWriter::pct_width(max_value, total_value);

        let max_key_width = data.iter().fold(0, |max, p| cmp::max(max, p.key().len()));
        let max_token_width = format!("{}", max_value).len();

        debug!(
            "[width={}; key={}; token={}; pct={}]",
            self.width, max_key_width, max_token_width, max_pct_width
        );
        let content_width = max_key_width + 1 + max_token_width + 1 + max_pct_width + 1 + 1;
        let bar_width = self.width.checked_sub(content_width).unwrap_or(0);

        let mut stderr = io::stderr();
        let c = ColumnWidths {
            key: max_key_width,
            token: max_token_width,
            pct: max_pct_width,
        };

        self.write_header(&mut stderr, c)?;

        for (i, p) in data.iter().enumerate() {
            let pct = p.value() as f64 / total_value as f64 * 100.0f64;

            write!(writer, "{:>width$}", p.key(), width = max_key_width)?;
            write!(writer, "{}", self.settings.regular_colour())?;
            write!(writer, "|")?;
            write!(writer, "{}", self.settings.ct_colour())?;
            write!(writer, "{:>width$}", p.value(), width = max_token_width)?;
            write!(writer, " ")?;

            // A good way to ensure padding is applied is to format your input,
            // then use this resulting string to pad your output.
            // https://doc.rust-lang.org/std/fmt/
            write!(writer, "{}", self.settings.pct_colour())?;
            write!(writer, "{:>width$}", format!("({:2.2}%)", pct), width = max_pct_width)?;

            write!(writer, "{}", self.settings.graph_colour())?;
            write!(writer, " {}", self.histogram_bar(max_value, bar_width, p.value()))?;

            if i == output_limit - 1 {
                writeln!(writer, "{}", self.settings.regular_colour())?;
            } else {
                writeln!(writer, "{}", self.settings.key_colour())?;
            }
        }
        Ok(())
    }

    fn histogram_bar(&self, max_value: u64, bar_width: usize, bar_value: u64) -> String {
        let zero_char: char;
        let one_char: char;
        let histogram_char = self.settings.histogram_char();
        let char_width = self.settings.char_width();
        if char_width < 1.0 {
            zero_char = *self.settings.graph_chars().last().expect("graph_chars is empty");
            one_char = '\0'
        } else if histogram_char.len() > 1 && !self.settings.unicode_mode() {
            zero_char = histogram_char.chars().nth(0).unwrap();
            one_char = histogram_char.chars().nth(1).unwrap();
        } else {
            zero_char = histogram_char.chars().nth(0).expect("histogram_char is empty");
            one_char = zero_char;
        }

        let width = (bar_value as f64) / (max_value as f64) * (bar_width as f64);
        let int_width = width.floor() as usize;
        let rem = width - int_width as f64;
        let graph_char = vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

        let mut bar = zero_char.to_string().repeat(int_width);

        if (char_width - 1.0).abs() < std::f64::EPSILON {
            bar.push(one_char);
        } else if char_width < 1.0 && rem > char_width {
            let which = (rem / char_width).floor() as usize;
            bar.push(graph_char[which])
        }

        bar
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use pairlist::Pair;
    use settings::Settings;

    use std::io::Cursor;

    macro_rules! args {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                temp_vec.push("test".to_string());
                temp_vec.push("--rcfile=/dev/null".to_string());
                $(
                    temp_vec.push($x.to_string());
                )*
                temp_vec.into_iter()
            }
        };
    }

    #[test]
    fn histogram_new() {
        let s = Settings::new(args![]);
        let h = HistogramWriter::new(s);
        assert_eq!(h.height, 15);
        assert_eq!(h.width, 80);
    }

    #[test]
    fn write_header() {
        let mut buff = Cursor::new(Vec::new());

        let s = Settings::new(args!["--width=10"]);
        let h = HistogramWriter::new(s);
        let c = ColumnWidths { key: 3, token: 3, pct: 3 };

        h.write_header(&mut buff, c).unwrap();

        let header = String::from_utf8_lossy(buff.get_ref());

        assert_eq!(header, "Key| Ct (Pct) Histogram\n---|------\n");
    }

    #[test]
    fn histogram_bar_one_char() {
        let s = Settings::new(args![]);
        let h = HistogramWriter::new(s);
        let bar = h.histogram_bar(16, 32, 8);
        assert_eq!(bar, "-----------------");
    }

    #[test]
    fn histogram_bar_two_char() {
        let s = Settings::new(args!["--char==>"]);
        let h = HistogramWriter::new(s);
        let bar = h.histogram_bar(16, 32, 8);
        assert_eq!(bar, "================>");
    }

    #[test]
    fn histogram_bar_partial_block() {
        let s = Settings::new(args!["--char=pb"]);
        let h = HistogramWriter::new(s);
        let bar = h.histogram_bar(100, 10, 55);
        assert_eq!(bar, "█████▋");
    }

    #[test]
    fn write_histogram_empty() {
        let s = Settings::new(args!["--graph=kv", "--width=15"]);
        let h = HistogramWriter::new(s);

        let mut counts: Vec<Pair> = Vec::new();
        let mut buf = io::Cursor::new(Vec::new());
        h.write_histogram(&mut buf, &mut counts).unwrap();

        let hist = String::from_utf8_lossy(buf.get_ref());

        assert_eq!(hist, "");
    }

    #[test]
    fn write_histogram_two_tokens() {
        let s = Settings::new(args!["--graph=kv", "--width=15"]);
        let h = HistogramWriter::new(s);

        let mut counts: Vec<Pair> = vec![Pair::new(1, "a"), Pair::new(2, "b")];
        let mut buf = io::Cursor::new(Vec::new());
        h.write_histogram(&mut buf, &mut counts).unwrap();

        let hist = String::from_utf8_lossy(buf.get_ref());

        assert_eq!(hist, "b|2 (66.67%) --\na|1 (33.33%) -\n");
    }
}
