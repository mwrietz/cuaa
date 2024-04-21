#![allow(dead_code)]

use crossterm::{
    cursor, execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
};
use getch::Getch;
use std::env;
use std::io::{stdout, Write};

pub struct MsgLine {
    pub msg: String,
    pub color: Color,
}

pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn clear_line() {
    execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
}

pub fn cursor_move(x: usize, y: usize) {
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

pub fn horiz_line(color: Color) {
    let (width, _) = tsize();
    for _i in 0..width {
        print_color_bold("─", color);
    }
    println!();
}

pub fn pause() {
    let (w, h) = tsize();
    let clear_message = "                            ";
    let message = "Press any key to continue...";
    let message_len: usize = message.len();
    cursor_move((w - message_len) / 2, h - 2);
    print_color(message, Color::DarkBlue);
    std::io::stdout().flush().unwrap();
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
    cursor_move((w - message_len) / 2, h - 2);
    print!("{}", clear_message);
}

pub fn print_color(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str),
        ResetColor
    )
    .expect("print_color error");
}

pub fn print_color_bold(my_str: &str, color: Color) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(my_str.bold()),
        ResetColor
    )
    .expect("print_color_bold error");
}

pub fn print_title(title_string: &str, color: Color) {
    println!();
    for c in title_string.chars() {
        print!(" ");
        print_color_bold(&c.to_string(), color);
    }
    println!();
    horiz_line(color);
    println!();
}

// *****************************
// splash() usage:
//
// use crossterm::style::Color;
// mod tui_gen;
// use tui_gen::MsgLine;
//
// -----------------------------
//
// let orange = Color::Rgb { r: 255, g: 135, b: 0 };
//
// let mut msg: Vec<MsgLine> = vec![];
// msg.push( MsgLine{ msg: "P R O G R A M   N A M E".to_string(), color: Color::DarkBlue} );
// msg.push( MsgLine{ msg: "".to_string(), color: Color::White} );
// msg.push( MsgLine{ msg: "v0.1.3".to_string(), color: orange } );
//
// tui_gen::splash(msg);
//
// *****************************

pub fn splash(msglines: Vec<MsgLine>) {
    cls();
    let (width, height) = tsize();
    let num_lines = msglines.len();

    let mut line_pos = height / 2 - num_lines / 2;

    for line in msglines {
        let line_len = line.msg.len();
        cursor_move(width / 2 - line_len / 2, line_pos);
        print_color_bold(&line.msg, line.color);
        line_pos += 1;
    }

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
    let dur = std::time::Duration::new(2, 0);
    std::thread::sleep(dur);
    cls();

    execute!(stdout(), cursor::Show).unwrap();
}

pub fn splash_screen(line1: &str, line2: &str) {
    cls();
    let (width, height) = tsize();

    let line1_length: usize = line1.len();
    cursor_move(width / 2 - line1_length / 2, height / 2 - 1);
    print_color_bold(line1, Color::DarkBlue);

    let line2_length: usize = line2.len();
    cursor_move(width / 2 - line2_length / 2, height / 2 + 1);
    print_color_bold(
        line2,
        Color::Rgb {
            r: 255,
            g: 135,
            b: 0,
        },
    );

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
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
            cursor_move(0, 0);
        }
    }
}

pub fn timestamp() -> String {
    let now = chrono::Local::now();
    now.to_string()
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
