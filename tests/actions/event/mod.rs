mod key;
mod mouse;

use enki::actions::{parse_event, ParseEventError};

fn assert_array_equals(s: &str, array: Vec<&str>) -> Result<(), ParseEventError> {
    let ev = parse_event(s)?;

    for item in array {
        assert_eq!(Ok(ev), parse_event(item));
    }
    Ok(())
}

fn modifier_tests(s: &str) -> Result<(), ParseEventError> {
    assert_array_equals(
        &format!("control shift alt {}", s),
        vec![
            &format!("control shift alt {}", s),
            &format!("alt control shift {}", s),
            &format!("{} alt control shift", s),
            &format!("{} control shift alt", s),
            &format!("alt {} shift control", s),
            &format!("control shift {} alt", s),
        ],
    )?;
    assert_array_equals(
        &format!("control alt {}", s),
        vec![
            &format!("control alt {}", s),
            &format!("alt control {}", s),
            &format!("{} alt control", s),
            &format!("{} control alt", s),
            &format!("alt {} control", s),
            &format!("control {} alt", s),
        ],
    )?;
    assert_array_equals(
        &format!("control {}", s),
        vec![&format!("control {}", s), &format!("{} control", s)],
    )?;
    assert_array_equals(
        &format!("alt {}", s),
        vec![&format!("alt {}", s), &format!("{} alt", s)],
    )?;
    assert_array_equals(
        &format!("shift {}", s),
        vec![&format!("shift {}", s), &format!("{} shift", s)],
    )?;
    Ok(())
}
