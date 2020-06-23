use crossterm::event::Event;
use enki::actions::parse_event;

use super::modifier_tests;

#[test]
fn parse() {
    modifier_tests("down(middlebtn)").unwrap();
    modifier_tests("up(middlebtn)").unwrap();
    modifier_tests("down(leftbtn)").unwrap();
    modifier_tests("up(leftbtn)").unwrap();
    modifier_tests("down(rightbtn)").unwrap();
    modifier_tests("up(rightbtn)").unwrap();
    modifier_tests("scroll_up").unwrap();
    modifier_tests("scroll_down").unwrap();

    let input = "resize";
    let result = Ok(Event::Resize(0, 0));
    assert_eq!(result, parse_event(input));
}
