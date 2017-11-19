use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;

#[derive(Debug,Default)]
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
    graph_values: String,
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

    pub fn graph_values(&self) -> &str {
        self.graph_values.as_str()
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

    pub fn new(args: env::Args) -> Settings {
        let mut s: Settings = Default::default();

        // non-zero defaults
        s.char_width = 1.0;
        s.match_regexp = String::from(r".");
        s.width = 80;
        s.height = 15;
        s.colour_palette = String::from("0,0,32,35,34");
        s.histogram_char = String::from("-");

        let mut opts: Vec<String> = args.collect();
        let rcfile = if opts.len() > 1 && opts[1].starts_with("--rcfile") {
            let idx = opts[1].find("=").unwrap();
            let (_, rcfile) = opts[1].split_at(idx + 1);
            String::from(rcfile)
        } else {
            let mut home = match env::home_dir() {
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
                    opts.insert(1, String::from(rcopt))
                }
            }
        }


        // manual argument parsing
        for arg in opts {
            if arg == "-h" || arg == "--help" {
                panic!("No usage yet!");
            } else if arg == "-c" || arg == "--color" {
                s.colourised_output = true;
            } else if arg == "-g" || arg == "--graph" {
                // can pass --graph without option, will default to value/key ordering
                // since unix perfers that for piping-to-sort reasons
                // TODO: replace strings w/ ENUMs
                s.graph_values = String::from("vk");
            } else {
                let v: Vec<&str> = arg.splitn(2, "=").collect();
                if v[0] == "-w" || v[0] == "--width" {
                    let w = v[1].parse::<usize>().unwrap();
                    s.width_arg = w;
                } else if v[0] == "-h" || v[0] == "--height" {
                    s.height_arg = v[1].parse::<usize>().unwrap();
                } else if v[0] == "-c" || v[0] == "--char" {
                    s.histogram_char = String::from(v[1]);
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
}
