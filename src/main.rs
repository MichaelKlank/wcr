use std::env;
use std::fs;
use std::io::{self, Read};

#[derive(Default)]
struct Options {
    count_bytes: bool,
    count_lines: bool,
    count_words: bool,
    count_chars: bool,
}

impl Options {
    fn is_any_set(&self) -> bool {
        self.count_bytes || self.count_lines || self.count_words || self.count_chars
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let (options, filename) = parse_args(&args);
    let content = read_content(filename.as_deref()).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    });

    let stats = Statistics::from_content(&content);
    let filename_display = filename.as_deref().unwrap_or("");

    if !options.is_any_set() {
        // Default: show lines, words, bytes
        println!(
            "{} {} {} {}",
            stats.lines, stats.words, stats.bytes, filename_display
        );
    } else {
        let mut outputs = Vec::new();
        if options.count_lines {
            outputs.push(format!("{} {}", stats.lines, filename_display));
        }
        if options.count_words {
            outputs.push(format!("{} {}", stats.words, filename_display));
        }
        if options.count_bytes {
            outputs.push(format!("{} {}", stats.bytes, filename_display));
        }
        if options.count_chars {
            outputs.push(format!("{} {}", stats.chars, filename_display));
        }
        println!("{}", outputs.join("\n"));
    }
}

fn parse_args(args: &[String]) -> (Options, Option<String>) {
    let mut options = Options::default();
    let mut filename = None;

    for arg in args {
        match arg.as_str() {
            "-c" => options.count_bytes = true,
            "-l" => options.count_lines = true,
            "-w" => options.count_words = true,
            "-m" => options.count_chars = true,
            path if !path.starts_with('-') => filename = Some(arg.clone()),
            _ => {
                eprintln!("Unknown option: {}", arg);
                std::process::exit(1);
            }
        }
    }

    (options, filename)
}

fn read_content(filename: Option<&str>) -> Result<String, io::Error> {
    match filename {
        Some(path) => fs::read_to_string(path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read file '{}': {}", path, e),
            )
        }),
        None => {
            let mut content = String::new();
            io::stdin().read_to_string(&mut content)?;
            Ok(content)
        }
    }
}

struct Statistics {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

impl Statistics {
    fn from_content(content: &str) -> Self {
        Self {
            lines: content.lines().count(),
            words: content.split_whitespace().count(),
            bytes: content.len(),
            chars: content.chars().count(),
        }
    }
}
