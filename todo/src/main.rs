extern crate clap;
extern crate parse_int;

use clap::{App, Arg};
use parse_int::parse;
use std::io::Write;
use std::fs::File;
use std::env;

pub fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let matches = App::new("todo.rs")
        .version("0.1.0")
        .author("shantaram <me@shantaram.xyz>")
        .about("simple to-do list app")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File name to operate on."),
        )
        .get_matches();

    let mut line = String::new();
    let file = matches.value_of("file");
    let filename = match file {
        Some(x) => x,
        None => {
            print!("Enter file name: ");
            match stdin.read_line(&mut line) {
                Err(why) => panic!("Couldn't read from stdin: {}", why),
                Ok(_) => (),
            }
            line.trim()
        }
    };

    fn err(msg: &str) {
        println!("ERROR: {}", msg);
    }

    let result = match std::fs::read_to_string(filename) {
        Err(why) => {
            err(&why.to_string());
            String::new()
        },
        Ok(val) => val,
    };

    let mut _lines: Vec<&str> = result.split("\n").collect();

    let mut lines: Vec<String> = Vec::new();
    for line in _lines {
        if line.trim_end() != "" {
            lines.push(line.to_owned())
        };
    }

    fn print_tasks(lines: &Vec<String>) {
        println!("****** Tasks ******");
        for (i, line) in lines.iter().enumerate() {
            if line != "" {
                println!("{} {}", i, line)
            }
        }
    }

    loop {
        print_tasks(&lines);
        print!("> ");
        stdout.flush().expect("Flushing stdout failed!");
        let mut inp = String::new();
        match stdin.read_line(&mut inp) {
            Err(why) => panic!("Couldn't read from stdin: {}", why),
            Ok(n) => {
                if n == 0 {
                    break;
                }
            },
        };
        inp = inp.trim_end().to_owned();

        let cmd: Vec<&str> = inp.split(" ").collect();
        let action = cmd[0];
        if action == "add" {
            if cmd.len() >= 2 {
                let line = cmd[1..].join(" ").clone();
                lines.push(line);
            } else {
                err("Too few arguments to rm")
            }
        } else if action == "rm" {
            if cmd.len() >= 2 {
                let idx = match parse::<i64>(cmd[1]) {
                    Err(why) => {
                        println!("Error parsing int from cmd {}: {}", cmd[1], why);
                        -1
                    }
                    Ok(val) => val,
                };
                if idx > 0 && idx < lines.len() as i64 {
                    lines.remove(idx as usize);
                } else {
                    err("Invalid index specified for rm");
                }
            } else {
                err("Too few arguments to rm");
            }
        } else if action == "exit" {
            break;
        } else {
            if action != "" {
                println!("Error: command not found: {}", cmd[0])
            };
        }
    }

    let tmpdir = env::temp_dir();
    let tmpfile = tmpdir.join("file");

    let mut file = File::create(&tmpfile).unwrap();

    for line in lines {
        match writeln!(file, "{}", line) {
            Err(why) => panic!("Error writing to temp file: {}", why),
            Ok(_) =>()
        }
    };

    match std::fs::copy(&tmpfile, filename) {
        Err(why) => panic!("Error writing file: data possibly lost. {}", why),
        Ok(_) => println!("Saved {} successfully.", filename)
    };
}
