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

fn main() {

    'mainloop: loop {

        print_shell_description(get_username(), get_current_location());
        
        let mut buf = command_input();
        if buf.len() == 0 {
            continue;
        }
        //io::stdout().into_raw_mode().unwrap();
        //command_input();

        //execute_command(buf);
        break;
    }
    //println!("randome thing");
    
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
fn execute_direct_command(command: String) {
    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];

    let buf = single_command_vector(command);

    if buf.len() == 0 {
        println!("");
        return;
    }
    if(builtin_command_list.contains(&buf[0])){
        let output = execute_env_commands(buf);
        match output {
            Some(output) => {
                println!("{}",output);
            },
            None => {return;}
        }
        return;
    }
        
    let mut output = Command::new(&buf[0]);
    for i in (1..buf.len()) {
        output.arg(&buf[i]);
    }
    let child = output.spawn();
    match child {
        Ok(mut child) => {
            child.wait();
        },
        Err(_) => {
            println!("No such file or directory");
        }
    }
}

fn execute_first_command(first_command: String) -> Option<String>{
    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];
    let mut buf = single_command_vector(first_command);
    if(buf.len() == 0) {
        println!("cannont start with pipe");
        std::io::stdout().flush().unwrap();
        return None;
    }
    if(builtin_command_list.contains(&buf[0])){
        let output = execute_env_commands(buf);
        return output;
    }

    let mut result = String::new();

    let mut output = Command::new(&buf[0]);
    for i in (1..buf.len()) {
        output.arg(&buf[i]);
    }
    output.stdout(Stdio::piped());
    let mut child = match output.spawn() {
        Ok(child) => {
            child
        }
        Err(_) => {
            println!("failed to spawn child");
            return None;
        }
    };
    if let Some(mut child_out) = child.stdout.take() {
        child_out.read_to_string(&mut result);
    } else {
        println!("failed to write the output in pipe");
        return None;
    }
    child.wait();
    return Some(result);
}


fn execute_last_command(last_command: String,mut result: String) {

    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];
    let mut buf = single_command_vector(last_command);
    if(buf.len() == 0) {
        println!("cannont end with pipe");
        std::io::stdout().flush().unwrap();
        return;
    }
    if(builtin_command_list.contains(&buf[0])){
        match execute_env_commands(buf) {
            Some(output) => {
                println!("{}",output);
            }
            None => {
            }
        }
        return;
    }

    let mut output = Command::new(&buf[0]);
    for i in (1..buf.len()) {
        output.arg(&buf[i]);
    }
    output.stdin(Stdio::piped());
    let mut child = match output.spawn() {
        Ok(child) => {
            child
        },
        Err(_) => {
            println!("failed to spawn child");
            return;
        }
    };
    child.stdin = if let Some(mut child_in) = child.stdin.take() {
        child_in.write(result.as_bytes());
        Some(child_in)
    } else {
        None
    };
    child.wait();

}

fn execute_middle_command(command: String, mut result: String) -> Option<String> {

    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];
    let mut buf = single_command_vector(command);
    if(buf.len() == 0) {
        println!("empty command between pipes");
        std::io::stdout().flush().unwrap();
        return None;
    }
    if(builtin_command_list.contains(&buf[0])){
        let output = execute_env_commands(buf);
        return output;
    }

    let mut output = Command::new(&buf[0]);
    for i in (1..buf.len()) {
        output.arg(&buf[i]);
    }
    output.stdout(Stdio::piped());
    output.stdin(Stdio::piped());
    let mut child = output.spawn().unwrap();
    
    if let Some(mut child_in) = child.stdin.take() {
        child_in.write(result.as_bytes());
    } else {
        println!("not able to read result from previous pipe");
        return None;
    }
    result.clear();
    if let Some(mut child_out) = child.stdout.take() {
        child_out.read_to_string(&mut result);
    } else {
        println!("failed to write the output in pipe");
        return None;
    }
    child.wait();

    return Some(result);
}

fn execute_command(mut command_list:VecDeque<String>) {

    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];

    if command_list.len() >= 2 {

        let mut result = String::new();
        let first_command = command_list.pop_front().unwrap();
        let last_command = command_list.pop_back().unwrap();

        ///executing first command in piped_commands
        match execute_first_command(first_command) {
            Some(output) => {
                result = output;
            },
            None => {
                return;
            }
        }
        

        ///executing middle command in piped_commands
        for command in command_list {

            let output = execute_middle_command(command, result);
            match output {
                Some(output) => {
                    result = output;
                }
                None => {
                    return;
                }
            }
        }
        
        ////executing last command in piped_commands
        execute_last_command(last_command, result);
        
        

    }
    else {
        execute_direct_command(command_list[0].clone());
    }
}


fn execute_env_commands(buf:Vec<String>) ->Option<String> {
    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];
    if &buf[0] == &builtin_command_list[1] {

        match env::current_dir() {
            Ok(path) => {
                match path.to_str() {
                    Some(path) => {
                        return Some(String::from(path));
                    }
                    None => {
                        return Some(String::from("unknown"));
                    }
                }
            },
            Err(_) => {
                return Some(String::from("unknown"));
            }
        }
        
    } else if &buf[0] == &builtin_command_list[2] {
        handle_cd(buf);
        return None;

    } else {
        println!("{}","to implement".red_front());
        return None;
    }
}


fn handle_cd(buf:Vec<String>) {
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



fn single_command_vector(buf:String) -> Vec<String> {
    let buf = String::from(buf.trim());
    let buf: Vec<String> = buf.split_whitespace().map(|args|  String::from(args.trim())).collect();
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


