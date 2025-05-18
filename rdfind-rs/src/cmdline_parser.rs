// Ported from: orig_src/CmdlineParser.hh and orig_src/CmdlineParser.cc
// Ported on 2025-05-05.

use std::env;
use std::process;

pub struct Parser {
    args: Vec<String>,
    index: usize,
    last_bool_result: bool,
    last_str_result: String,
}

impl Parser {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        Parser {
            args,
            index: 1, // skip program name
            last_bool_result: false,
            last_str_result: String::new(),
        }
    }

    pub fn try_parse_bool(&mut self, arg: &str) -> bool {
        if self.index >= self.args.len() {
            eprintln!(
                "out of bounds: index={} argc={}",
                self.index,
                self.args.len()
            );
            process::exit(1);
        }
        if arg == self.args[self.index] {
            if self.index + 1 >= self.args.len() {
                eprintln!(
                    "expected true or false after {} not end of argument list.",
                    arg
                );
                process::exit(1);
            }
            let value = &self.args[self.index + 1];
            if value == "true" {
                self.last_bool_result = true;
                self.index += 1;
                return true;
            }
            if value == "false" {
                self.last_bool_result = false;
                self.index += 1;
                return true;
            }
            eprintln!("expected true or false after {} not '{}'.", arg, value);
            process::exit(1);
        }
        false
    }

    pub fn try_parse_string(&mut self, arg: &str) -> bool {
        if self.index >= self.args.len() {
            eprintln!(
                "out of bounds: index={} argc={}",
                self.index,
                self.args.len()
            );
            process::exit(1);
        }
        if arg == self.args[self.index] {
            if self.index + 1 >= self.args.len() {
                eprintln!("expected string after {} not end of argument list.", arg);
                process::exit(1);
            }
            self.last_str_result = self.args[self.index + 1].clone();
            self.index += 1;
            return true;
        }
        false
    }

    pub fn get_parsed_bool(&self) -> bool {
        self.last_bool_result
    }

    pub fn get_parsed_string(&self) -> &str {
        &self.last_str_result
    }

    pub fn parsed_string_is(&self, value: &str) -> bool {
        self.last_str_result == value
    }

    pub fn advance(&mut self) -> usize {
        self.index += 1;
        self.index
    }

    pub fn has_args_left(&self) -> bool {
        self.index < self.args.len()
    }

    pub fn get_current_index(&self) -> usize {
        self.index
    }

    pub fn get_current_arg(&self) -> &str {
        if self.index >= self.args.len() {
            eprintln!(
                "out of bounds: index={} argc={}",
                self.index,
                self.args.len()
            );
            process::exit(1);
        }
        &self.args[self.index]
    }

    pub fn current_arg_is(&self, what: &str) -> bool {
        self.get_current_arg() == what
    }
}
