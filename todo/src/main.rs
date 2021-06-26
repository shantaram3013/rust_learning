extern crate clap;
extern crate parse_int;

use clap::{App, Arg};
use parse_int::parse;
use std::env;
use std::fs::File;
use std::io::Write;

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
            stdout.flush().expect("Flushing stdout failed!");
            stdin.read_line(&mut line).expect("Error reading filename.");
            line.trim()
        }
    };

    fn err(msg: &str) {
        eprintln!("ERROR: {}", msg);
    }

    let result = match std::fs::read_to_string(filename) {
        Err(why) => {
            err(&why.to_string());
            String::new()
        }
        Ok(val) => val,
    };
    
    let mut lines: Vec<String> = Vec::new();
    for line in result.split('\n') {
        if !line.trim_end().is_empty() {
            lines.push(line.to_owned())
        };
    }

    fn print_tasks(lines: &[String]) {
        println!("****** Tasks ******");
        for (i, line) in lines.iter().enumerate() {
            println!("{} {}", i, line)
        }
    }

    let mut inp = String::new();
    loop {
        print_tasks(&lines);
        print!("> ");
        stdout.flush().expect("Flushing stdout failed!");
        inp.clear();
        match stdin.read_line(&mut inp) {
            Err(why) => panic!("Couldn't read from stdin: {}", why),
            Ok(n) => { // handle Ctrl-D
                if n == 0 {
                    break;
                }
            }
        };

        let cmd: Vec<&str> = inp.trim_end().split_whitespace().collect();
        let action = cmd[0];

        match action {
            "add" => {
                let line = cmd[1..].join(" ").clone();
                if cmd.len() >= 2 {
                    if line.is_empty() {
                        continue;
                    }
                    lines.push(line);
                } else {
                    err("Too few arguments to rm")
                }
            }
            "rm" => {
                if cmd.len() >= 2 {
                    match parse::<usize>(cmd[1]) {
                        Err(why) => {
                            println!("Error parsing int from cmd {}: {}", cmd[1], why);
                        }
                        Ok(val) => {
                            if val > 0 && val < lines.len() {
                                lines.remove(val);
                            } else {
                                err("Invalid index specified for rm");
                            }
                        }
                    };
                } else {
                    err("Too few arguments to rm");
                }
            } 
            "exit" => {
                break;
            }
            "" => {
                /* pass */
            }
            _ => {
                println!("Error: command not found: {}", cmd[0])
            }
        }
    }

    let tmpdir = env::temp_dir();
    let tmpfile = tmpdir.join("file");

    let mut file = File::create(&tmpfile).unwrap();

    for line in lines {
        writeln!(file, "{}", line).expect("Error writing to temp file");
    }

    std::fs::copy(tmpfile, filename).expect("Error writing file: data possibly lost");
    println!("Saved {} successfully.", filename);
}
