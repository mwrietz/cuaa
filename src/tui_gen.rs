#![allow(dead_code)]

//use colored::Colorize;
use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
};
use getch::Getch;
use std::io::{stdout, Write};
use std::env;

fn clr(c: &str) -> Color {
    let c_upper: &str = &c.to_uppercase();
    match c_upper {
        "RED" => Color::Red,
        "BLUE" => Color::Blue,
        "CYAN" => Color::Cyan,
        "GREEN" => Color::Green,
        "GREY" => Color::Grey,
        "YELLOW" => Color::Yellow,
        "MAGENTA" => Color::Magenta,
        _ => Color::White,
    }
}

pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn cmove(x: usize, y: usize) {
    execute!(stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
}

pub fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}

pub fn horiz_line(color: &str) {
    for _i in 0..80 {
        //print!("{}", "─".color(color).bold());
        print_color_bold("─", color);
    }
    println!("");
}

pub fn pause() {
    let (w, h) = tsize();
    let clear_message = "                            ";
    //let message = "Press any key to continue...".blue();
    let message = "Press any key to continue...";
    let message_len: usize = message.len();
    cmove((w - message_len) / 2, h - 2);
    //print!("{}", message);
    print_color(message, "BLUE");
    std::io::stdout().flush().unwrap();
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
    cmove((w - message_len) / 2, h - 2);
    print!("{}", clear_message);
}

pub fn print_color(my_str: &str, color: &str) {
    execute!(
        stdout(),
        SetForegroundColor(clr(color)),
        Print(my_str),
        ResetColor
    ).expect("print_color error");
}

pub fn print_color_bold(my_str: &str, color: &str) {
    execute!(
        stdout(),
        SetForegroundColor(clr(color)),
        Print(my_str.bold()),
        ResetColor
    ).expect("print_color_bold error");
}

pub fn print_title(title_string: &str, color: &str) {
    println!("");
    for c in title_string.chars() {
        print!("{}", " ");
        //print!("{}", c.to_string().color(color).bold());
        print_color_bold(&c.to_string(), color);
    }
    println!("");
    horiz_line(color);
    println!("");
}

pub fn splash_screen(line1: &str, line2: &str) {
    //const VERSION: &str = env!("CARGO_PKG_VERSION");

    cls();
    let (width, height) = tsize();

    let line1_length: usize = line1.len();
    cmove(width / 2 - line1_length / 2, height / 2 - 1);
    //println!("{}", line1.bold());
    print_color_bold(line1, "WHITE");

    let line2_length: usize = line2.len();
    cmove(width / 2 - line2_length / 2, height / 2 + 1);
    println!("{}", line2);

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
    //let one_sec = std::time::Duration::from_millis(1000);
    let dur = std::time::Duration::new(2, 0);
    std::thread::sleep(dur);
    cls();

    execute!(stdout(), cursor::Show).unwrap();
}

//
// TermStat usage:
// let mut termstat = TermStat::default();
//

pub struct TermStat {
    pub line_count: usize,
    pub width: usize,
    pub height: usize,
    pub xpos: usize,
    pub ypos: usize,
}

impl Default for TermStat {
    fn default() -> TermStat {
        let (w, h) = tsize();
        let (x, y) = tpos();
        TermStat {
            line_count: 0,
            width: w,
            height: h,
            xpos: x,
            ypos: y,
        }
    }
}

impl TermStat {
    pub fn line_check(&mut self) {
        let (_x, y) = tpos();
        if y > (self.height - 5) {
            pause();
            cls();
            cmove(0, 0);
        }
    }
}

pub fn timestamp() -> String {
    let now = chrono::Local::now();
    return now.to_string();
}

pub fn tpos() -> (usize, usize) {
    let pos = crossterm::cursor::position();
    let (x, y) = match pos {
        Ok((x, y)) => (x, y),
        Err(error) => panic!("tpos error: {:?}", error),
    };
    (x as usize, y as usize)
}

pub fn tsize() -> (usize, usize) {
    let size = crossterm::terminal::size();
    let (w, h) = match size {
        Ok((w, h)) => (w, h),
        Err(error) => panic!("tsize error: {:?}", error),
    };
    (w as usize, h as usize)
}
