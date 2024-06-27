#![allow(warnings)]
extern crate text_styler;
use std::io::{stdin,Write};
use std::process::Command;

use text_styler::TextStyler;
fn main() {


    'mainloop: loop {

        let username:String = match Command::new("whoami").output() { //everytime calculating username since program might change username
            Ok(output) => {
                match String::from_utf8(output.stdout) {
                    Ok(username) => String::from(username.trim()),
                    Err(_) => String::from("username"),
                }
            }
            Err(_) => String::from("username"),
        };  
    
    
        let current_location:Vec<String>= match Command::new("pwd").output() { //everytime calculating current location since program might change current working directory
            Ok(output) => {
                match String::from_utf8(output.stdout) {
                    Ok(current_location) => String::from(current_location.trim()).split("/").map(|x| String::from(x)).collect(),
                    Err(_) => vec![String::from("unknown")],
                }
            }
            Err(_) => vec![String::from("unknown")],
        }; 

        let mut username_directory = String::from("[");
        username_directory += username.as_str();
        username_directory += " ";
        username_directory += match (current_location.last()) {
            Some(last_value) => {
                last_value
            },
            None => "unknown",
        };
        username_directory += "/]$ ";


        print!("{}",username_directory.green_front().bold());
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        stdin().read_line(&mut buf);
        buf = String::from(buf.trim());
        if buf == String::from("exit") {
            break 'mainloop;
        }
        // println!("{:?}",buf.bytes());
        // return;
        //let x:Vec<u8> = buf.bytes().collect();
        //println!("{:?}",x);
        //break;
        let mut output = Command::new(buf);
        let hello = output.output().expect("faild to execute the program");
        println!("{:#?}",hello);
    }
    
}
