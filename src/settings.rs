use dirs;

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process;

#[derive(Debug, PartialEq)]
pub enum PreTallied {
    NA,
    KeyValue,
    ValueKey,
}

impl Default for PreTallied {
    fn default() -> PreTallied {
        PreTallied::NA
    }
}

#[derive(Debug, Default)]
pub struct Settings {
    program_name: String,
    total_millis: u32,
    start_time: i64,
    end_time: i64,
    width_arg: usize,
    height_arg: usize,
    width: usize,
    height: usize,
    histogram_char: String,
    colourised_output: bool,
    logarithmic: bool,
    num_only: String,
    verbose: bool,
    graph_values: PreTallied,
    size: String,
    tokenize: String,
    match_regexp: String,
    stat_interval: i32,
    num_prunes: u32,
    colour_palette: String,
    regular_colour: String,
    key_colour: String,
    ct_colour: String,
    pct_colour: String,
    graph_colour: String,
    total_objects: u32,
    total_values: u64,
    key_prune_interval: u32,
    max_keys: u32,
    unicode_mode: bool,
    char_width: f64,
    graph_chars: Vec<char>,
    partial_blocks: Vec<String>,
    partial_lines: Vec<String>,
}

impl Settings {
    // getters
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn graph_values(&self) -> &PreTallied {
        &self.graph_values
    }

    pub fn tokenize(&self) -> &str {
        self.tokenize.as_str()
    }

    pub fn match_regexp(&self) -> &str {
        self.match_regexp.as_str()
    }

    pub fn char_width(&self) -> f64 {
        self.char_width
    }

    pub fn graph_chars(&self) -> &[char] {
        &self.graph_chars
    }

    pub fn histogram_char(&self) -> &str {
        self.histogram_char.as_str()
    }

    pub fn unicode_mode(&self) -> bool {
        self.unicode_mode
    }

    pub fn regular_colour(&self) -> &str {
        self.regular_colour.as_str()
    }

    pub fn key_colour(&self) -> &str {
        self.key_colour.as_str()
    }

    pub fn ct_colour(&self) -> &str {
        self.ct_colour.as_str()
    }

    pub fn pct_colour(&self) -> &str {
        self.pct_colour.as_str()
    }

    pub fn graph_colour(&self) -> &str {
        self.graph_colour.as_str()
    }

    pub fn new<I>(args: I) -> Settings
    where
        I: Iterator<Item = String>,
    {
        let mut s: Settings = Default::default();

        // non-zero defaults
        s.program_name = Settings::get_program_name().unwrap();
        s.char_width = 1.0;
        s.match_regexp = String::from(r".");
        s.width = 80;
        s.height = 15;
        s.colour_palette = String::from("0,0,32,35,34");
        s.histogram_char = String::from("-");

        let mut opts: Vec<String> = args.collect();
        // FIXME rcfile may not be first passed argument
        let rcfile = if opts.len() > 1 && opts[1].starts_with("--rcfile") {
            let idx = opts[1].find("=").unwrap();
            let (_, rcfile) = opts[1].split_at(idx + 1);
            String::from(rcfile)
        } else {
            let mut home = match dirs::home_dir() {
                Some(h) => h,
                None => panic!("No home directory for user!"),
            };
            home.push(".distributionrc");
            String::from(home.to_str().unwrap())
        };

        if let Ok(f) = File::open(rcfile) {
            let file = BufReader::new(&f);
            for line in file.lines() {
                let l = line.unwrap();
                let rcopt = match l.find("#") {
                    Some(idx) => {
                        let (first, _) = l.split_at(idx);
                        String::from(first)
                    }
                    None => l,
                };
                if rcopt != "" {
                    opts.insert(0, String::from(rcopt))
                }
            }
        }

        // manual argument parsing
        for arg in opts {
            if arg == "-h" || arg == "--help" {
                s.do_usage(&mut io::stdout()).expect("error printing usage");
                process::exit(1);
            } else if arg == "-c" || arg == "--color" {
                s.colourised_output = true;
            } else if arg == "-g" || arg == "--graph" {
                // can pass --graph without option, will default to value/key ordering
                // since unix perfers that for piping-to-sort reasons
                // TODO: replace strings w/ ENUMs
                s.graph_values = PreTallied::ValueKey;
            } else {
                let v: Vec<&str> = arg.splitn(2, "=").collect();
                if v[0] == "-w" || v[0] == "--width" {
                    let w = v[1].parse::<usize>().unwrap();
                    s.width_arg = w;
                } else if v[0] == "-h" || v[0] == "--height" {
                    s.height_arg = v[1].parse::<usize>().unwrap();
                } else if v[0] == "-c" || v[0] == "--char" {
                    s.histogram_char = String::from(v[1]);
                } else if v[0] == "-g" || v[0] == "--graph" {
                    s.graph_values = match v[1] {
                        "vk" => PreTallied::ValueKey,
                        "kv" => PreTallied::KeyValue,
                        _ => panic!("Invalid graph value"),
                    }
                } else if v[0] == "-p" || v[0] == "--palette" {
                    s.colour_palette = String::from(v[1]);
                    s.colourised_output = true;
                } else if v[0] == "-s" || v[0] == "--size" {
                    s.size = String::from(v[1])
                } else if v[0] == "-t" || v[0] == "--tokenize" {
                    s.tokenize = String::from(v[1])
                } else if v[0] == "-m" || v[0] == "--match" {
                    s.match_regexp = String::from(v[1])
                }
            }
        }

        // first, size, which might be further overridden by width/height later
        if s.size == "small" || s.size == "sm" || s.size == "s" {
            s.width = 60;
            s.height = 10;
        } else if s.size == "medium" || s.size == "med" || s.size == "m" {
            s.width = 100;
            s.height = 20;
        } else if s.size == "large" || s.size == "lg" || s.size == "l" {
            s.width = 140;
            s.height = 35;
        }

        // override variables if they were explicitly given
        if s.width_arg != 0 {
            s.width = s.width_arg;
        }

        if s.height_arg != 0 {
            s.height = s.height_arg;
        }

        // colour palette
        if s.colourised_output {
            let cl: Vec<&str> = s.colour_palette.splitn(5, ",").collect();
            s.regular_colour = format!("\u{001b}[{}m", cl[0]);
            s.key_colour = format!("\u{001b}[{}m", cl[1]);
            s.ct_colour = format!("\u{001b}[{}m", cl[2]);
            s.pct_colour = format!("\u{001b}[{}m", cl[3]);
            s.graph_colour = format!("\u{001b}[{}m", cl[4]);
        }

        if s.histogram_char == "dt" {
            s.unicode_mode = true;
            s.histogram_char = "•".to_string();
        }

        if s.histogram_char == "pb" {
            s.char_width = 0.125;
            s.graph_chars = vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        }

        // detect whether the user has passed a multibyte unicode character
        // directly as the histogram char
        if s.histogram_char.as_bytes()[0] > 128 {
            s.unicode_mode = true
        }

        // println!("rcfile: {:?}", s);
        s
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn do_usage<T: io::Write>(&self, writer: &mut T) -> io::Result<()> {
        writeln!(writer)?;
        writeln!(writer, "usage: <commandWithOutput> | {}", self.program_name)?;
        writeln!(writer, "         [--size={{sm|med|lg|full}} | --width=<width> --height=<height>]")?;
        writeln!(writer, "         [--color] [--palette=r,k,c,p,g]")?;
        writeln!(writer, "         [--Tokenize=<tokenChar>]")?;
        writeln!(writer, "         [--graph[=[kv|vk]] [--numonly[=derivative,diff|abs,absolute,actual]]")?;
        writeln!(writer, "         [--char=<barChars>|<substitutionString>]")?;
        writeln!(writer, "         [--help] [--verbose]")?;
        writeln!(writer, "  --keys=K       every {} values added, prune hash to K keys (default 5000)", self.key_prune_interval)?;
        writeln!(writer, "  --char=C       character(s) to use for histogram character, some substitutions follow:")?;
        writeln!(writer, "        pl       Use 1/3-width unicode partial lines to simulate 3x actual terminal width")?;
        writeln!(writer, "        pb       Use 1/8-width unicode partial blocks to simulate 8x actual terminal width")?;
        writeln!(writer, "        ba       (▬) Bar")?;
        writeln!(writer, "        bl       (Ξ) Building")?;
        writeln!(writer, "        em       (—) Emdash")?;
        writeln!(writer, "        me       (⋯) Mid-Elipses")?;
        writeln!(writer, "        di       (♦) Diamond")?;
        writeln!(writer, "        dt       (•) Dot")?;
        writeln!(writer, "        sq       (□) Square")?;
        writeln!(writer, "  --color        colourise the output")?;
        writeln!(writer, "  --graph[=G]    input is already key/value pairs. vk is default:")?;
        writeln!(writer, "        kv       input is ordered key then value")?;
        writeln!(writer, "        vk       input is ordered value then key")?;
        writeln!(writer, "  --height=N     height of histogram, headers non-inclusive, overrides --size")?;
        writeln!(writer, "  --help         get help")?;
        writeln!(writer, "  --logarithmic  logarithmic graph")?;
        writeln!(writer, "  --match=RE     only match lines (or tokens) that match this regexp, some substitutions follow:")?;
        writeln!(writer, "        word     ^[A-Z,a-z]+\\$ - tokens/lines must be entirely alphabetic")?;
        writeln!(writer, "        num      ^\\d+\\$        - tokens/lines must be entirely numeric")?;
        writeln!(writer, "  --numonly[=N]  input is numerics, simply graph values without labels")?;
        writeln!(writer, "        actual   input is just values (default - abs, absolute are synonymous to actual)")?;
        writeln!(writer, "        diff     input monotonically-increasing, graph differences (of 2nd and later values)")?;
        writeln!(writer, "  --palette=P    comma-separated list of ANSI colour values for portions of the output")?;
        writeln!(writer, "                 in this order: regular, key, count, percent, graph. implies --color.")?;
        writeln!(writer, "  --rcfile=F     use this rcfile instead of ~/.distributionrc - must be first argument!")?;
        writeln!(writer, "  --size=S       size of histogram, can abbreviate to single character, overridden by --width/--height")?;
        writeln!(writer, "        small    40x10")?;
        writeln!(writer, "        medium   80x20")?;
        writeln!(writer, "        large    120x30")?;
        writeln!(writer, "        full     terminal width x terminal height (approximately)")?;
        writeln!(writer, "  --Tokenize=RE  split input on regexp RE and make histogram of all resulting tokens")?;
        writeln!(writer, "        word     [^\\w] - split on non-word characters like colons, brackets, commas, etc")?;
        writeln!(writer, "        white    \\s    - split on whitespace")?;
        writeln!(writer, "  --width=N      width of the histogram report, N characters, overrides --size")?;
        writeln!(writer, "  --verbose      be verbose")?;
        writeln!(writer)?;
        writeln!(writer, "You can use single-characters options, like so: -h=25 -w=20 -v. You must still include the =")?;
        writeln!(writer)?;
        writeln!(writer, "Samples:")?;
        writeln!(writer, "  du -sb /etc/* | {} --palette=0,37,34,33,32 --graph", self.program_name)?;
        writeln!(writer, "  du -sk /etc/* | awk '{{print $2\" \"$1}}' | {} --graph=kv", self.program_name)?;
        writeln!(writer, "  zcat /var/log/syslog*gz | {} --char=o --Tokenize=white", self.program_name)?;
        writeln!(writer, "  zcat /var/log/syslog*gz | awk '{{print \\$5}}'  | {} -t=word -m-word -h=15 -c=/", self.program_name)?;
        writeln!(writer, "  zcat /var/log/syslog*gz | cut -c 1-9        | {} -width=60 -height=10 -char=em", self.program_name)?;
        writeln!(writer, "  find /etc -type f       | cut -c 6-         | {} -Tokenize=/ -w=90 -h=35 -c=dt", self.program_name)?;
        writeln!(writer, "  cat /usr/share/dict/words | awk '{{print length(\\$1)}}' | {} -c=* -w=50 -h=10 | sort -n", self.program_name)?;
        writeln!(writer)?;
        Ok(())
    }

    pub fn get_program_name() -> Result<String, String> {
        let current_exe: PathBuf = match env::current_exe() {
            Ok(path) => path,
            Err(err) => return Err(err.to_string()),
        };

        let file_name: &OsStr = match current_exe.file_name() {
            Some(name) => name,
            None => return Err("oh no!".to_string()),
        };

        match file_name.to_str() {
            Some(name) => Ok(String::from(name)),
            None => return Err("oh no!".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{PreTallied, Settings};

    #[test]
    fn test_empty_args() {
        let args: Vec<String> = Vec::new();
        let s = Settings::new(args.into_iter());

        // check non-zero defaults
        assert_eq!(s.width(), 80);
        assert_eq!(s.height(), 15);
    }

    macro_rules! test_option {
        ($name:ident, $opt:expr, $($f:ident, $v:expr),+) => {
            #[test]
            fn $name () {
                let args = vec!["test".to_string(), "--rcfile=/dev/null".to_string(), $opt.to_string()];
                let s = Settings::new(args.into_iter());

                $(assert_eq!(s.$f, $v);)*
            }
        };
    }

    macro_rules! test_option_fail {
        ($name:ident, $opt:expr) => {
            #[test]
            #[should_panic]
            fn $name() {
                let args = vec![$opt.to_string()];

                Settings::new(args.into_iter());
            }
        };
    }

    test_option!(
        rcfile,
        "--rcfile=/dev/null",
        char_width,
        1.0,
        match_regexp,
        r".",
        width,
        80,
        height,
        15,
        colour_palette,
        "0,0,32,35,34",
        histogram_char,
        "-"
    );

    test_option!(no_color, "", colourised_output, false);
    test_option!(short_color, "-c", colourised_output, true);
    test_option!(long_color, "--color", colourised_output, true);

    test_option!(not_graph, "", graph_values, PreTallied::NA);
    test_option!(short_graph, "-g", graph_values, PreTallied::ValueKey);
    test_option!(long_graph, "--graph", graph_values, PreTallied::ValueKey);
    test_option!(long_graph_vk, "--graph=vk", graph_values, PreTallied::ValueKey);
    test_option!(short_graph_kv, "--graph=kv", graph_values, PreTallied::KeyValue);
    test_option_fail!(invalid_graph, "--graph=foo");

    test_option!(short_width, "-w=40", width, 40);
    test_option!(long_width, "--width=60", width, 60);
    test_option_fail!(invalid_short_width, "-w=abc");
    test_option_fail!(invalid_long_width, "--width=xyz");

    test_option!(short_height, "-h=77", height, 77);
    test_option!(long_height, "--height=113", height, 113);
    test_option_fail!(invalid_short_height, "-h=abc");
    test_option_fail!(invalid_long_height, "--height=xyz");

    test_option!(short_char, "-c=-", histogram_char, "-");
    test_option!(long_char, "--char=x", histogram_char, "x");
    test_option!(char_dt, "--char=dt", histogram_char, "•", unicode_mode, true);
    test_option!(
        char_pb,
        "--char=pb",
        histogram_char,
        "pb",
        char_width,
        0.125,
        graph_chars,
        vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█']
    );
    test_option!(
        char_unicode,
        "--char=\u{2652}",
        histogram_char,
        "\u{2652}",
        char_width,
        1.0,
        unicode_mode,
        true
    );

    test_option!(
        short_palette,
        "-p=0,37,34,33,32",
        colour_palette,
        "0,37,34,33,32",
        colourised_output,
        true
    );
    test_option!(
        long_palette,
        "--palette=0,37,34,33,32",
        colour_palette,
        "0,37,34,33,32",
        colourised_output,
        true
    );
    test_option_fail!(invalid_short_palette, "-p=x");
    test_option_fail!(invalid_long_palette, "--palette=x");

    test_option!(short_size_small, "-s=small", size, "small", width, 60, height, 10);
    test_option!(long_size_small, "--size=small", size, "small", width, 60, height, 10);
    test_option!(short_size_sm, "-s=sm", size, "sm", width, 60, height, 10);
    test_option!(long_size_sm, "--size=sm", size, "sm", width, 60, height, 10);
    test_option!(short_size_s, "-s=s", size, "s", width, 60, height, 10);
    test_option!(long_size_s, "--size=s", size, "s", width, 60, height, 10);

    test_option!(short_size_medium, "-s=medium", size, "medium", width, 100, height, 20);
    test_option!(long_size_medium, "--size=medium", size, "medium", width, 100, height, 20);
    test_option!(short_size_med, "-s=med", size, "med", width, 100, height, 20);
    test_option!(long_size_med, "--size=med", size, "med", width, 100, height, 20);
    test_option!(short_size_m, "-s=m", size, "m", width, 100, height, 20);
    test_option!(long_size_m, "--size=m", size, "m", width, 100, height, 20);

    test_option!(short_tokenize, "-t=(.)", tokenize, "(.)");
    test_option!(long_tokenize, "--tokenize=(.)", tokenize, "(.)");

    test_option!(short_match, "-m=(.)", match_regexp, "(.)");
    test_option!(long_match, "--match=(.)", match_regexp, "(.)");
}
