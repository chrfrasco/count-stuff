// cs - Count Stuff
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Flags {
    lines: bool,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => print_usage(),
        _ => run(args),
    }
}

fn print_usage() {
    println!("usage: cs [-l] [file ...]

options:
    -l  Count the number of lines in the input
")
}

fn run(args: Vec<String>) {
    let mut flags = Flags {
        lines: false,
    };
    let mut files: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            parse_flag_arg(&arg, &mut flags);
        } else {
            files.push(arg);
        }
    }

    if flags.lines {
        println!("{}", line_count(files))
    }
}

fn parse_flag_arg(flag_arg: &str, flags: &mut Flags) {
    for character in flag_arg.chars() {
        match character {
            'l' => flags.lines = true,
            _ => continue,
        }
    }
}

fn line_count(files: Vec<String>) -> usize {
    if files.len() == 0 {
        let stdin = io::stdin();
        stdin.lock().lines().count()
    } else {
        files.iter().map(File::open).fold(0, |sum, file| match file {
            Ok(f) => sum + BufReader::new(f).lines().count(),
            Err(_) => sum,
        })
    }
}
