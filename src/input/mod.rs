use std::{collections::VecDeque, env, ffi::{c_int, c_long, CStr}, io::{self, stdin, Read, Write}, mem::take, os::fd::AsRawFd, process::{exit, Child, Command, Stdio}, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::{self, Duration}};

use crate::lib::TextStyler;

/*
typedef unsigned char	cc_t;
typedef unsigned int	speed_t;
typedef unsigned int	tcflag_t;
#define NCCS 32



tcflag_t c_iflag;		/* input mode flags */
tcflag_t c_oflag;		/* output mode flags */
tcflag_t c_cflag;		/* control mode flags */
tcflag_t c_lflag;		/* local mode flags */
cc_t c_line;			/* line discipline */
cc_t c_cc[NCCS];		/* control characters */
speed_t c_ispeed;		/* input speed */
speed_t c_ospeed;		/* output speed */
*/
//https://www.man7.org/linux/man-pages/man3/termios.3.html
#[repr(C)]
struct Termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

fn clone(original_termios: &Termios) -> Termios {
    let mut termios: Termios = Termios {
        c_iflag : original_termios.c_iflag,
        c_oflag : original_termios.c_oflag,
        c_cflag : original_termios.c_cflag,
        c_lflag : original_termios.c_lflag,
        c_line : original_termios.c_line,
        c_cc : original_termios.c_cc.clone(),
        c_ispeed : original_termios.c_ispeed,
        c_ospeed : original_termios.c_ospeed,
        
    };
    return termios;
}

//c_lflag flag constants:
const ECHO: u32 = 8;
const ICANON: u32 = 2;

const ESCAPE_CHAR: u8 = '/' as u8;
const TAB_CHAR: u8 = '\t' as u8;
const NEW_LINE: u8 = '\n' as u8;
const CARRIAGE_RETURN: u8 = '\r' as u8;
const BACKSPACE: u8 = '\u{7f}' as u8;
const INTERRUPT_EXIT: u8 = '\u{3}' as u8;
const SIGKILL: i32 = 9;
//const UP_ARROW: u8 = '\u{1b}[A' as u8;

extern "C" {
    fn printf(var: *const std::os::raw::c_char, ...) -> i32;
    fn cfmakeraw(var: Termios);
    fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *mut Termios) -> i32;
    fn fork() -> i32;
    fn getpid() -> i32;
    fn getppid() -> i32;
    fn kill(pid: i32, sig: i32) -> i32;
    fn waitpid(pid: i32, statloc: *mut i32, options: i32) -> i32;
}
/*
int tcgetattr(int fd, struct termios *termios_p);
int tcsetattr(int fd, int optional_actions, const struct termios *termios_p);
*/

pub fn command_input()  {

    let mut termios;
    let mut original_termios;
    /// terminal settings
    unsafe {
        termios = std::mem::zeroed();
        tcgetattr(0, &mut termios);
        original_termios = clone(&termios);
        termios.c_lflag = (!ECHO)|(!ICANON);
        tcsetattr(0, 0, &mut termios);
    }

    /// declaration of required variables
    let mut buf = Arc::new(Mutex::new(String::new()));
    let mut c: [u8; 1] = [0];
    let mut stdout = io::stdout();
    let mut child: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
    

    /// loop to take input
    loop {

        /// taking character by character input
        stdin().read_exact(&mut c).expect("unable to read".red_front().as_str());

        /// processing character by character
        match c {
            /// handelling ctrl+c
            [INTERRUPT_EXIT] => {
                {
                    let mut mutex_child = &mut *child.lock().unwrap(); 
                    kill_child(mutex_child);
                }
                {
                    let mut buf_copy = buf.lock().unwrap();
                    (*buf_copy).clear();
                }
                write!(stdout,"\n");
                stdout.flush();
                print_shell_description(get_username(), get_current_location());
            },
            /// handelling backspace
            [BACKSPACE] => {
                let mut buf_string = buf.lock().unwrap();
                if buf_string.len() == 0 {
                    continue;
                } else {
                    buf_string.pop();
                    write!(stdout,"{}", ('\u{8}' as char));
                    write!(stdout," ");
                    write!(stdout,"{}", ('\u{8}' as char));
                    stdout.flush();
                }
            },
            /// when pressed enter check for child process
            [CARRIAGE_RETURN] => {
                //first check child process
                write!(stdout,"\n");
                stdout.flush();
                let mut child_clone = Arc::clone(&child);
                let piped_command = Arc::clone(&buf);
                let mut command_string = String::new();
                {
                    let mut buf_string = buf.lock().unwrap();
                    command_string = (*buf_string).clone();
                    (*buf_string).clear();
                }
                thread::spawn(move || {
                    run_command(child_clone,separate_pipes(command_string));
                    println!("this");
                    print_shell_description(get_username(), get_current_location());
                });
            },
            /// TODO tab for autocomplete and suggest
            [TAB_CHAR] => {},
            /// pushing other characters into buffer string
            [c] => {
                write!(stdout,"{}",c as char);
                stdout.flush();
                {
                    let mut buf_string = buf.lock().unwrap();
                    (*buf_string).push(c as char);
                }
            },
        }
    }

    unsafe {
        tcsetattr(0, 0, &mut original_termios);
    }

}


fn separate_pipes(buf: String) -> VecDeque<String> {
    let buf: VecDeque<String> = buf.split("|").map(|args| String::from(args.trim())).collect();
    return buf;
}


fn kill_child( mutex_child: &mut Option<Child>) {

    match mutex_child {
        Some(child_process) => {
            child_process.kill();
            return;
        }
        None => {
            return;
        }
    }
}

fn execute_command(mut command_list:VecDeque<String>,some_child: Arc<Mutex<Option<Child>>>) {


    let  builtin_command_list = [String::from("exit"),String::from("pwd"),String::from("cd"),String::from("export"),String::from("unset")];

    if command_list.len() >= 2 {

        // let mut result = String::new();
        // let first_command = command_list.pop_front().unwrap();
        // let last_command = command_list.pop_back().unwrap();

        // ///executing first command in piped_commands
        // match execute_first_command(first_command) {
        //     Some(output) => {
        //         result = output;
        //     },
        //     None => {
        //         return;
        //     }
        // }
        // ///executing middle command in piped_commands
        // for command in command_list {

        //     let output = execute_middle_command(command, result);
        //     match output {
        //         Some(output) => {
        //             result = output;
        //         }
        //         None => {
        //             return;
        //         }
        //     }
        // }
        
        // ////executing last command in piped_commands
        // execute_last_command(last_command, result);
        
        

    }
    else 
    {
        execute_direct_command(command_list[0].clone(),some_child);
        println!("executed direct");
    }
    //exit(0);
}

fn execute_first_command(first_command: String, some_child: Arc<Mutex<Option<Child>>>) -> Option<String>{
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
            // {
            //     let mut mutex_child = &mut *some_child.lock().unwrap();
            //     *mutex_child = Some(child);
            //     //(*mutex_child).wait();
            // }
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


fn execute_direct_command(command: String,some_child: Arc<Mutex<Option<Child>>>) {

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
    //println!("trying to acquire child lock");     
        
    let mut output = Command::new(&buf[0]);
    for i in (1..buf.len()) {
        output.arg(&buf[i]);
    }
    let child = output.spawn();
    match child {
        Ok(mut child) => {
            {
                let mut mutex_child = &mut *some_child.lock().unwrap();
                *mutex_child = Some(child);
                //rem this
                match (&mut *mutex_child) {
                    Some(ch) => {
                        ch.wait();
                    },
                    None => {
                        return;
                    }
                }
            }
            //let x = child.wait();
            //child.wait();
        },
        Err(_) => {
            println!("No such file or directory");
        }
    }
}

fn single_command_vector(buf:String) -> Vec<String> {
    let buf = String::from(buf.trim());
    let buf: Vec<String> = buf.split_whitespace().map(|args|  String::from(args.trim())).collect();
    return buf;
}

fn run_command(some_child: Arc<Mutex<Option<Child>>>, command_list: VecDeque<String>) {
    execute_command(command_list,some_child);
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
        exit(0);
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