use colored::*;
use std::io::*;

pub struct Console;

impl Console {
    pub fn prompt (question: &str) -> String {
        print!("{} ", question);
        stdout ().flush ().expect ("Failed to flush Stdout");

        let stdin = stdin();
        let mut handle = stdin.lock ();
        let mut answer = String::new ();

        handle.read_line (&mut answer).expect ("Failed to read from Stdin");
        answer.pop ();

        println!("\x1b[1A{} {}", question, answer.cyan ());
        stdout ().flush ().expect ("Failed to flush Stdout");
        return answer;
    }
}