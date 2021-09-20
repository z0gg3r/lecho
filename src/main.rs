/*   SPDX-FileCopyrightText: 2021 zocker <zockerfreunde03.info@gmx.de
 *
 *   SPDX-License-Identifier: GPL-3.0-or-later
 *
 *   lecho - A program to echo a specific line from a file
 *   Copyright (C) 2021  zockerfreunde03/z0gg3r
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License Version 3 or
 *   later as published by the Free Software Foundation.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License or the LICENSE file for more details.
 *
 *   You should have received a copy of the GNU General Public License along
 *   with this program; if not, write to the Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#[macro_use]
extern crate clap;
use clap::{App, Arg, ArgGroup};

struct Response {
        cont: String,
        err: bool,
}

impl Response {
        fn ok(s: String) -> Response {
                Response {
                        cont: s,
                        err: false,
                }
        }

        fn err(s: String) -> Response {
                Response { cont: s, err: true }
        }

        fn is_ok(&self) -> bool {
                !self.err
        }

        fn unwrap(&self) -> String {
                self.cont.clone()
        }
}

fn main() {
        let matches = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .arg(Arg::with_name("infile")
                        .short("f")
                        .long("-file")
                        .takes_value(true)
                        .help("The file to read"))
                .arg(Arg::with_name("line")
                        .takes_value(true)
                        .help("The line to echo")
                        .short("-l")
                        .long("-line"))
                .arg(Arg::with_name(" line")
                        .takes_value(true)
                        .help("The line to echo"))
                .arg(Arg::with_name("csv_mode")
                        .short("-c")
                        .long("-csv")
                        .help("Treat input as Commaseperated Values"))
                .arg(Arg::with_name("delimiter")
                        .short("-d")
                        .long("-delmiter")
                        .takes_value(true)
                        .help("The delimter that is used (Implies -c)"))
                .arg(Arg::with_name("index").short("-i").long("-index").takes_value(true).help(
                        "Which field of the CSV line to print (Default: 2 | Implies -c)",
                ))
                .group(ArgGroup::with_name("lines")
                        .args(&["line", " line"])
                        .required(true))
                .get_matches();

        let file = matches.value_of("infile").unwrap_or("").to_string();
        let line = matches.value_of("lines").unwrap_or("Error").to_string();
        let csv = matches.is_present("csv_mode")
                || matches.is_present("delimiter")
                || matches.is_present("index");
        let delim = matches.value_of("delimiter").unwrap_or(",");
        let index = matches.value_of("index").unwrap_or("2");
        let index: usize = index.parse().unwrap_or_else(|_| {
                eprintln!("Please provide a valid positive integer (i >= 1)");
                0
        });
        if index == 0 {
                // Don't do anything.
        } else if file.is_empty() {
                // Reading from pipe adapted from:
                // https://stackoverflow.com/a/49734144
                let mut v: Vec<String> = vec![];

                loop {
                        let mut input = String::new();
                        let bytes = std::io::stdin()
                                .read_line(&mut input)
                                .expect("Failed to read from stdin!");
                        input = input.trim().to_string();
                        // If we read 0 bytes we are at EOF and stop
                        // reading
                        if bytes == 0 {
                                break;
                        }

                        v.push(input.clone());
                }

                let line_number: usize = line
                        .parse()
                        .unwrap_or_else(|_| {
                            eprintln!("Please provide a valid line number (i >= 1). Note: to exit the line number will be set to the length of the file + 1");
                            v.len() + 1
                        });

                if line_number <= v.len() {
                        if csv {
                                let line: Vec<&str> = v[line_number - 1]
                                        .split(delim)
                                        .collect();
                                println!("{}", line[index - 1]);
                        } else {
                                println!("{}", v[line_number - 1]);
                        }
                } else {
                        eprintln!(
                                "Cannot print line {} from {} lines of input!",
                                line_number,
                                v.len()
                        );
                }
        } else {
                let content = read_file(file);
                if content.is_ok() {
                        let lines = parse(content.unwrap());
                        let line_number: usize = line
                            .parse()
                            .unwrap_or_else(|_| {
                                    eprintln!("Please provide a valid line number (i >= 1). Note: To exit the line number will be set to the length of the file + 1");
                                    lines.len() + 1
                            });
                        if !line_number > lines.len() {
                                if csv {
                                        let val: Vec<&str> = lines
                                                [line_number - 1]
                                                .split(delim)
                                                .collect();
                                        println!(
                                                "{}",
                                                val[index - 1].to_string()
                                        );
                                } else {
                                        println!("{}", lines[line_number - 1]);
                                }
                        } else {
                                eprintln!(
                    "{} is too big. The file only has {} lines!",
                    line_number,
                    lines.len()
                );
                        }
                } else {
                        eprintln!(
                "There were some errors while reading the file: {}",
                content.unwrap()
            );
                }
        }
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
                Response::err(format!(
                        "Cannot find file {}. Does it exist?",
                        _path
                ))
        } else if !path.is_file() {
                Response::err(format!(
                        "{} exits, but does not seem to be a file.",
                        _path
                ))
        } else {
                let file = std::fs::read_to_string(path);
                if file.is_ok() {
                        Response::ok(file.unwrap())
                } else {
                        Response::err(file.unwrap_err().to_string())
                }
        }
}
