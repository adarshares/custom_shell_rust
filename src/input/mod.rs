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
    let mut buf = String::new();
    let mut c: [u8; 1] = [0];
    let mut stdout = io::stdout();
    let mut child: Option<Child> = None;
    

    /// loop to take input
    loop {

        /// taking character by character input
        stdin().read_exact(&mut c).expect("unable to read".red_front().as_str());

        /// processing character by character
        match c {
            /// handelling ctrl+c
            [INTERRUPT_EXIT] => {
                buf.clear();
                match child.take() {
                    Some (mut child_process)=> {
                        child_process.kill().expect("cannot kill child process");
                        child_process.wait();
                    },
                    None => {}
                };
                write!(stdout,"\n");
                stdout.flush();
                print_shell_description(get_username(), get_current_location());
            },
            /// handelling backspace
            [BACKSPACE] => {
                if buf.len() == 0 {
                    continue;
                } else {
                    buf.pop();
                    write!(stdout,"{}", ('\u{8}' as char));
                    write!(stdout," ");
                    write!(stdout,"{}", ('\u{8}' as char));
                    stdout.flush();
                }
            },
            /// when pressed enter check for child process
            [CARRIAGE_RETURN] => {
                write!(stdout,"\n");
                stdout.flush();
                if(buf.trim() == String::from("exit")){
                    break;
                }
                child = match child.take() {
                    Some(mut child_process) => {
                        match child_process.try_wait() {
                            Ok(Some(_)) => {
                                //exited zombie
                                child_process.kill();
                                child_process.wait();
                                None
                            }
                            Ok(None) => {
                                //still running
                                Some(child_process)
                            }
                            Err(_) => {
                                //
                                None
                            }
                        }
                        //continue;
                    },
                    None => {
                        None
                    }
                };

                child = Some(Command::new("target/debug/child").arg(buf).spawn().unwrap());
                buf = String::new();
            },
            /// TODO tab for autocomplete and suggest
            [TAB_CHAR] => {},
            /// pushing other characters into buffer string
            [c] => {
                write!(stdout,"{}",c as char);
                stdout.flush();
                buf.push(c as char);
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