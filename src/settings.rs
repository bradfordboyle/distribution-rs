use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::fs::File;

#[derive(Default)]
pub struct Settings {
    program_name: String,
    total_millis: u32,
    start_time: i64,
    end_time:          i64,
    width_arg:         usize,
    height_arg:        usize,
    width:            usize,
    height:           usize,
    histogram_char:    String,
    colourised_output: bool,
    logarithmic:      bool,
    num_only:          String,
    verbose:          bool,
    graph_values:      String,
    size:             String,
    tokenize:         String,
    match_regexp:      String,
    stat_interval:     i32,
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
    char_width: f32,
    graph_chars: Vec<String>,
    partial_blocks: Vec<String>,
    partial_lines: Vec<String>
}

impl Settings {
    // getters
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn new(args: env::Args) -> Settings {
        let mut s: Settings = Default::default();

        let mut opts: Vec<String> = args.collect();
        let rcfile = if opts.len() > 1 && opts[1].starts_with("--rcfile") {
            let idx = opts[1].find("=").unwrap();
            let (_, rcfile) = opts[1].split_at(idx+1);
            String::from(rcfile)
        } else {
            let mut home = match env::home_dir() {
                Some(h) => h,
                None => panic!("No home directory for user!"),
            };
            home.push(".distributionrc");
            String::from(home.to_str().unwrap())
        };

        let f = File::open(rcfile).unwrap();
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line.unwrap();
            let rcopt = match l.find("#") {
                Some(idx) => {
                    let (first, _) = l.split_at(idx);
                    String::from(first)
                },
                None => l
            };
            if rcopt != "" {
                opts.insert(1, String::from(rcopt))
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
                }
            }
        }

        // override variables if they were explicitly given
        if s.width_arg != 0 {
            s.width = s.width_arg;
        }

        if s.height_arg != 0 {
            s.height = s.height_arg;
        }

        println!("rcfile: {:?}", s.width);
        s
    }
}
