// Lib using termcolor crate to print lines with color.
use std::io::{stdin, Write, BufRead};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// Write text in yellow.
pub fn write_yellow(text:String){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
    writeln!(&mut stdout, "{}", text);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "{}", "");
}

// Write text in green.
pub fn write_green(text:String){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
    writeln!(&mut stdout, "{}", text);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "{}", "");
}

// Write text in red.
pub fn write_red(text:String){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
    writeln!(&mut stdout, "{}", text);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "{}", "");
}

// Write text in cyan.
pub fn write_cyan(text:String){
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
    writeln!(&mut stdout, "{}", text);
    let _ =  stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
    writeln!(&mut stdout, "{}", "");
}
