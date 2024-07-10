#![allow(warnings)]
extern crate text_styler;
use std::io::{stdin, Error, Write};
use std::process::Command;
use std::env;

use text_styler::TextStyler;

// enum builtin_command_list {
//     cd,
//     exit

// }

fn main() {

    
    
    // cd, exit, export, pwd and unset
    let builtin_command_list = vec![String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];
    'mainloop: loop {

        print_shell_description(get_username(), get_current_location());
        
        let mut buf = command_input();
        if buf.len() == 0 {
            continue;
        }

        if builtin_command_list.contains(&buf[0]) {
            if &buf[0] == &builtin_command_list[0] {
                break 'mainloop;
            } else if &buf[0] == &builtin_command_list[1] {

                match env::current_dir() {
                    Ok(path) => {
                        match path.to_str() {
                            Some(path) => {
                                println!("{path}");
                            }
                            None => {
                                println!("unknown");
                            }
                        }
                    },
                    Err(_) => {
                        println!("unknown");
                    }
                }
                
            } else if &buf[0] == &builtin_command_list[2] {

                handle_cd(buf);

            } else {
                println!("{}","to implement".red_front());
            }

            
        }
        else {
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
    
}

fn handle_cd(buf:Vec<String>) {
    // let x = env::current_dir();
    // println!("{:#?}",x);
    // env::set_current_dir("./../../");
    // let x = env::current_dir();
    // println!("{:#?}",x);
    // env::set_current_dir("/");
    // let x = env::current_dir();
    // println!("{:#?}",x);
    if(buf.len() == 1) {
        return;
    }
    match env::set_current_dir(&buf[1]) {
        Ok(_) => {}
        Err(_) => {
            println!("{}","No such directory/file exist".red_front());
        }
    }
}

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

fn command_input() -> Vec<String> {

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect(("cannot read from commandline".red_front().bold()).as_str());
    buf = String::from(buf.trim());
        
    let buf :Vec<String> = buf.split_ascii_whitespace().map(|args|  String::from(args.trim())).collect();
    return buf;

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


