use super::modifier_tests;

#[test]
fn parse() {
    let chars = "abcdefghijklmnopqrstuvwxyz";

    let digits = "0123456789";

    for chr in chars.chars() {
        modifier_tests(&format!("{}", chr)).unwrap();
    }

    for chr in chars.to_uppercase().chars() {
        modifier_tests(&format!("{}", chr)).unwrap();
    }

    for chr in digits.chars() {
        modifier_tests(&format!("{}", chr)).unwrap();
    }

    for num in 0..19 {
        modifier_tests(&format!("f{}", num)).unwrap();
        modifier_tests(&format!("F{}", num)).unwrap();
    }

    modifier_tests("home").unwrap();
    modifier_tests("end").unwrap();
    modifier_tests("pageup").unwrap();
    modifier_tests("pagedown").unwrap();
    modifier_tests("backtab").unwrap();
    modifier_tests("tab").unwrap();
    modifier_tests("delete").unwrap();
    modifier_tests("backspace").unwrap()
}
