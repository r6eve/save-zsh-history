#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

macro_rules! errorln {
    () => ({
        eprintln!("error");
        ::std::process::exit(1);
    });
    ($fmt:expr) => ({
        eprintln!($fmt);
        ::std::process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        ::std::process::exit(1);
    });
}

fn doit<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    lazy_static! {
        // Set unsave commands using regular expression.
        // The following show a example.
        static ref ARG_CMD: Regex = Regex::new(
            r"(?x)
            ^:\s\d+:\d+;(?:
                \./.+$|
                7z|
                ag|
                caddy|
                cat|
                chmod|
                cp|
                g\+\+|
                gcc|
                git\sadd|
                mplayer\s[^~-].+$|
                tail)\s"
        ).unwrap();

        // Save commands if pipe is used.
        static ref PIPE_CMD: Regex = Regex::new(
            r"(?x)
            ^:\s\d+:\d+;(?:.+\|.+$)"
        ).unwrap();
    }

    let tmpfile = "/tmp/for_zsh_history";

    {
        let in_ = File::open(&path)?;
        let in_ = BufReader::new(in_);
        let mut out = File::create(&tmpfile)?;

        for line in in_.lines() {
            let line = match line {
                Err(..) => continue,
                Ok(l) => l,
            };
            if PIPE_CMD.is_match(&line) {
                let line = line + "\n";
                out.write_all(line.as_bytes())?;
                continue;
            }
            if ARG_CMD.is_match(&line) {
                continue;
            }
            let line = line + "\n";
            out.write_all(line.as_bytes())?;
        }
    }

    fs::copy(&tmpfile, &path)?;
    fs::remove_file(&tmpfile)?;

    Ok(())
}

fn main() {
    // Set your `.zsh_history` path.
    let file = ".zsh_history";
    doit(file).unwrap_or_else(|e| errorln!("{:?}", e));
}
