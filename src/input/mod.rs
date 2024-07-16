use std::{collections::VecDeque, io::stdin, process::exit};

use crate::lib::TextStyler;

pub fn command_input() -> VecDeque<String> {

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect(("cannot read from commandline".red_front().bold()).as_str());
    buf = String::from(buf.trim());
    // let buf = String::from("cat Cargo.lock | uniq | wc -l");
    if buf == String::from("exit") {
        exit(0);
    }
        
    let buf :VecDeque<String> = buf.split("|").map(|args|  String::from(args.trim())).collect();
    return buf;

}