#![allow(warnings)]
extern crate text_styler;
use std::io::{stdin, Error, Write};
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
        username_directory += match current_location.last() {
            Some(last_value) => {
                last_value
            },
            None => "unknown",
        };
        username_directory += "/]$ ";
        
        
        print!("{}",username_directory.green_front().bold());
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();
        stdin().read_line(&mut buf).expect(("cannot read from commandline".red_front().bold()).as_str());
        buf = String::from(buf.trim());
        if buf == String::from("exit") {
            break 'mainloop;
        }
        
        let buf :Vec<String> = buf.split_ascii_whitespace().map(|args|  String::from(args.trim())).collect();
        
        let mut output = Command::new(&buf[0]);
        for i in (1..buf.len()) {
            output.arg(&buf[i]);
        }
        match output.output() {
            Ok(output) => {
                match String::from_utf8(output.stdout.clone()) {
                    Ok(output) => {print!("{}",output);},
                    Err(_)=> println!("{:#?}",output),
                }
            },
            Err(err) => {
                match err {
                    //std::io::ErrorKind::NotFound => {}
                    _  => {println!("error running program or program does not exist")}
                }
            }
        }
    }
    
}
// let custom = Error::new(std::io::ErrorKind::AddrInUse, "rand");
// println!("{:#?}",custom);

// match custom {
//     ee => {
//         let kind_of_error = ee.kind();
//         match ee.raw_os_error() {
//             Some(ee) => {
//                 println!("{:#?}",ee);
//             }
//             None => {
//                 println!("{} \nType: {}","Error found".red_front().bold(),kind_of_error)
//             }
//         }
//     }
// }

// return;


