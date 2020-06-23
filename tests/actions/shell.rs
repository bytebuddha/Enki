use enki::actions::Action;

use super::parse_action;

fn shell_test(s: &str, out: &str) {
    let output = out.split(' ').map(ToString::to_string).collect();
    assert_eq!(Ok(Action::ShellCommand(output)), parse_action(s));
}

#[test]
fn parse() {
    shell_test("-- ls -lsa", "ls -lsa");
    shell_test("-- lsblk", "lsblk");
    shell_test("-- ls -lsa | grep thing", "ls -lsa | grep thing");
    shell_test("-- ls -lsa ; find thing", "ls -lsa ; find thing");
}
