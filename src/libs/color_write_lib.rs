// Lib using termcolor crate to print lines with color.
use std::io::{stdin, Write, BufRead};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Write text colorized.
pub fn write_color(text:String, is_same_line:bool, color:Color){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(color)));
    if is_same_line {
        write!(&mut stdout, "{}", text);
    }else {
        writeln!(&mut stdout, "{}", text);
    }
    let _ =  stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "{}", "");
}

