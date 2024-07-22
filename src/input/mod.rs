use std::{collections::VecDeque, ffi::{c_int, CStr}, io::{self, stdin, Read, Write}, os::fd::AsRawFd, process::exit, thread::JoinHandle};

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
}
/*
int tcgetattr(int fd, struct termios *termios_p);
int tcsetattr(int fd, int optional_actions, const struct termios *termios_p);
*/

pub fn command_input() -> VecDeque<String> {

    let mut termios;
    let mut original_termios;
    unsafe {
        termios = std::mem::zeroed();
        tcgetattr(0, &mut termios);
        original_termios = clone(&termios);
        termios.c_lflag = (!ECHO)|(!ICANON);
        tcsetattr(0, 0, &mut termios);
    }

    let mut args: Vec<String> = Vec::new();
    let mut buf = String::new();
    let mut c: [u8; 1] = [0];
    let mut last_char_escape: bool = false;
    let mut stdout = io::stdout();
    let mut child_pid = -1;

    buf.clear();
    loop {
        stdin().read_exact(&mut c).expect("unable to read".red_front().as_str());

        match c {
            [INTERRUPT_EXIT] => {
                match child_pid {
                    -1 => {
                        break;
                        
                    }
                    child_pic => {
                        
                    }
                }
                
            }
            [BACKSPACE] => {
                if buf.len() == 0 {
                    continue;
                } else {
                    //write!(stdout,"{}",BACKSPACE);
                    //write!(stdout,"{}",buf.len());
                    buf.pop();
                    write!(stdout,"{}", ('\u{8}' as char));
                    write!(stdout," ");
                    write!(stdout,"{}", ('\u{8}' as char));
                    stdout.flush();
                }
            }
            [CARRIAGE_RETURN] => {
                write!(stdout,"\n");
                stdout.flush();
                match option_handle {
                    Some(handle) => {
                        //handle.join();
                        buf.clear();
                        break;
                    }
                    None => {
                        //spawn thread
                        break;
                    }
                }
            },
            [TAB_CHAR] => {},
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
    //println!("{:#?}",buf);


    buf = String::from(buf.trim());
    // let buf = String::from("cat Cargo.lock | uniq | wc -l");
    // println!("{:#?}",buf);
    if buf == String::from("exit") {
        exit(0);
    }
    
        
    let buf :VecDeque<String> = buf.split("|").map(|args|  String::from(args.trim())).collect();
    //println!("{:#?}",buf);
    return buf;

}