use log::debug;
use tui::style::Color;

use std::io::Result;
use std::process::Command;

pub fn run_shell_command(cmd: Vec<String>) -> Result<String> {
    debug!("Running Shell Command: {}", cmd.join(" "));
    let output = if cmd.len() > 1 {
        Command::new(&cmd[0]).args(&cmd[1..]).output()?
    } else {
        Command::new(&cmd[0]).output()?
    };
    Ok(String::from_utf8_lossy(&output.stdout).into())
}

pub fn u32_to_color(argb_color: u32) -> Color {
    let r = ((argb_color & 0x00ff_0000) >> 16) as u8;
    let g = ((argb_color & 0x0000_ff00) >> 8) as u8;
    let b = (argb_color & 0x0000_00ff) as u8;
    Color::Rgb(r, g, b)
}
