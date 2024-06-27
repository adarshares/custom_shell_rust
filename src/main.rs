#![allow(warnings)]
extern crate text_styler;
use std::io::stdin;
use std::process::Command;

use text_styler::TextStyler;
fn main() {

    // let username = 
    //     vec:Command::new("uname").arg("-r").output().unwrap().stdout;

    // println!("{:?}",username);
    // return;

    'mainloop: loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf);
        if buf == String::from("exit") {
            break 'mainloop;
        }
        buf = String::from(buf.trim());
        //let x:Vec<u8> = buf.bytes().collect();
        //println!("{:?}",x);
        //break;
        let mut output = Command::new(buf);
        let hello = output.output().expect("faild to execute the program");
        println!("{:#?}",hello);
    }
    
}
