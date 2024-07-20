use std::{collections::VecDeque, ffi::{c_int, CStr}, io::{stdin, Read}, os::fd::AsRawFd, process::exit};

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

const TCSANOW: i32 = 0;
const ECHO: i32 = 8;
const ICANON: i32 = 2;


extern "C" {
    fn printf(var: *const std::os::raw::c_char, ...) -> i32;
    fn cfmakeraw(var: Termios);
    fn tcgetattr(fd: i32, optional_actions: i32, termios_p: *mut Termios) -> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *mut Termios) -> i32;
}
/*
int tcgetattr(int fd, struct termios *termios_p);
int tcsetattr(int fd, int optional_actions, const struct termios *termios_p);
*/

pub fn command_input() -> VecDeque<String> {

    unsafe {
        printf("abcdefg\n\n\0" as *const str as *const i8);
        let mut termios: Termios = std::mem::zeroed();
        cfmakeraw(termios);
    }

    let mut args: Vec<String> = Vec::new();
    let mut buf = String::new();
    let mut c: [u8; 1] = [0];

    let mut last_char_escape: bool = false;
    let escape_char = '/' as u8;
    let tab_char = '\t' as u8;




    loop {
        break;
        stdin().read_exact(&mut c).expect("unable to read".red_front().as_str());
        print!("afa\x08");
        // match c {
        //     [excape_char] => {
        //         print!("{:#?}",c);
        //     },
        //     [tab_char] => {},
        //     [_] => {
        //        // print!("{:#?}",_);
        //     }
        // }
        //print!("{:?}",c);
        break;
        
    }


    buf = String::from(buf.trim());
    // let buf = String::from("cat Cargo.lock | uniq | wc -l");
    if buf == String::from("exit") {
        exit(0);
    }
    
        
    let buf :VecDeque<String> = buf.split("|").map(|args|  String::from(args.trim())).collect();
    return buf;

}