// SPDX-FileCopyrightText: 2021 zocker <zockerfreunde03.info@gmx.de
//
// SPDX-License-Identifier: GPL-3.0-or-later

use getopt::Opt;

struct Response {
    cont: String,
    err: bool
}

impl Response {
    fn ok(s: String) -> Response {
        Response {
            cont: s,
            err: false
        }
    }

    fn err(s: String) -> Response {
        Response {
            cont: s,
            err: true
        }
    }

    fn is_ok(&self) -> bool {
        !self.err
    }

    fn unwrap(&self) -> String {
        self.cont.clone()
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = getopt::Parser::new(&args, "f:l:hcd:");

    let mut file = String::new();
    let mut line = String::new();
    let mut help = false;
    let mut csv = false;
    let mut delim = String::from(",");

    loop {
        match opts
                .next()
                .transpose()
                .expect("Failed to parse arguments! Please report this to the author/maintainer!")
            {
                None => break,
                Some(opt) => match opt {
                    Opt('f', Some(arg)) => file = arg.clone(),
                    Opt('l', Some(arg)) => line = arg.clone(),
                    Opt('h', None) => help = true,
                    Opt('c', None) => csv = true,
                    Opt('d', Some(arg)) => delim = arg.clone(),
                    _ => unreachable!(),
                },
            }
    }

        if !help && !file.is_empty() && !line.is_empty() {
            let line_number: usize = line.parse().expect("Please provide a valid line number!");
            let content = read_file(file);
            if content.is_ok() {
                let lines = parse(content.unwrap());
                if !line_number > lines.len() {
                    if csv { 
                        let val: Vec<&str> = lines[line_number - 1].split(&delim).collect();
                        println!("{}", val[1].to_string());
                    } else {
                        println!("{}", lines[line_number - 1]);
                    }
                } else {
                    eprintln!("{} is too big. The file only has {} lines!", line_number, lines.len());
                }
            } else {
                eprintln!("There were some errors while reading the file: {}", content.unwrap());
            }
        } else {
            print_usage();
        }
}

fn print_usage() {
    println!("lecho -f file -l line [-c] [-d delimiter]");
}

fn parse(s: String) -> Vec<String> {
    let lines: Vec<&str> = s.split('\n').collect();

    let mut v = Vec::new();

    for line in lines {
        v.push(String::from(line));
    }

    v
}

fn read_file(_path: String) -> Response {
   let path = std::path::Path::new(&_path);
   if !path.exists() {
       Response::err(format!("Cannot find file {}. Does it exist?", _path))
   } else if !path.is_file() {
       Response::err(format!("{} exits, but does not seem to be a file.", _path))
   } else {
       let file = std::fs::read_to_string(path);
       if file.is_ok() {
	   Response::ok(file.unwrap())
       } else {
	   Response::err(file.unwrap_err().to_string())
       }
   }
}
