#![allow(warnings)]
//extern crate text_styler;
use std::collections::VecDeque;
use std::io::{self, stdin, Error, Read, Write};
use std::process::{exit, Child, Command, Output, Stdio};
use std::env;
//use termion;

pub mod lib;
use lib::TextStyler;
pub mod input;
use input::command_input;
const INTERRUPT_STRING:&str = "\u{3}";

fn main() {

        print_shell_description(get_username(), get_current_location());
        command_input();
        //Command::new("./a.out").spawn().unwrap().wait();
    
    
}
// enum builtin_command_list {
//     cd,
//     exit
//implement internal trait on external
//ctrlk ctrl0
//ctrlk ctrlj
// } wc -l flaw enter enter enter 

/// executes the command if not has any pipe
/// takes input command as a string
/// if empty string then do nothing
/// if env command then calls execute env command
/// for rest commands spawns child process and waits for its finish


fn get_username() -> String { //everytime calculating username since program might change username
    match Command::new("whoami").output() { 
        Ok(output) => {
            match String::from_utf8(output.stdout) {
                Ok(username) => String::from(username.trim()),
                Err(_) => String::from("username"),
            }
        }
        Err(_) => String::from("username"),
    } 
}

fn get_current_location() -> Vec<String> { //everytime calculating current location since program might change current working directory
    match env::current_dir() { 
        Ok(output) => {
            match output.to_str() {
                Some(current_location) => {
                    return String::from (current_location.trim()).split("/").map(|x| String::from(x)).collect();
                },
                None => {
                    return vec![String::from("unknown")];
                }
            }
        }
        Err(_) => {return vec![String::from("unknown")];},
    }
}

fn print_shell_description(username: String,current_location: Vec<String> ) {
    let mut username_directory = String::from("[");
    username_directory += username.as_str();
    username_directory += " ";
    username_directory += match current_location.last() {
        Some(last_value) => {
            last_value
        },
        None => "unknown",
    };
    username_directory += "/]$ ";
    
    
    print!("{}",username_directory.green_front().bold());
    std::io::stdout().flush().unwrap();
}

