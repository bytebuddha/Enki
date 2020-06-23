use std::io;
use std::io::BufRead;
use std::io::BufReader;

use enki::xi::XiNotification;
use serde_json::from_str;

#[test]
fn parse_lines() -> io::Result<()> {
    let lines_file_path = include_str!("./notifications.txt");

    let mut reader = BufReader::new(lines_file_path.as_bytes());
    let mut counter = 1;
    loop {
        let mut buf = String::new();
        let count = reader.read_line(&mut buf)?;
        if count == 0 {
            break;
        }
        if let Err(err) = from_str::<XiNotification>(&buf) {
            println!("{}, err: {}", counter, buf);
            panic!("{}, {}", counter, err);
        }
        counter += 1;
    }
    Ok(())
}
