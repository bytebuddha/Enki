use serde_json::{json, Value};

use enki::enki::Configuration;

#[test]
fn get_set() {
    let mut config = Configuration::default();

    config.set("string", json!("value"));
    config.set("boolean", json!(true));
    config.set("number", json!(16));
    config.set("int_array", json!([1, 2, 3]));

    assert_eq!(Some(json!("value")), config.get("string"));
    assert_eq!(Some(json!(true)), config.get("boolean"));
    assert_eq!(Some(json!(16)), config.get("number"));
}

#[test]
fn get_default() {
    let mut config = Configuration::default();

    config.set("string", json!("Value"));
    config.set("boolean", Value::Bool(false));

    assert_eq!(json!("Value"), config.get_default("string", json!(true)));
    assert_eq!(json!(true), config.get_default("str", json!(true)));
    assert_eq!(json!(false), config.get_default("boolean", json!(true)));
}

#[test]
fn get_default_t() {
    let mut config = Configuration::default();

    config.set("string", json!("data"));
    config.set("bool", json!(true));
    config.set("number", json!(1234));

    assert_eq!(
        "data".to_string(),
        config.get_default_t("string", "g".to_string())
    );
    assert_eq!(true, config.get_default_t("bool", false));
    assert_eq!(false, config.get_default_t("bo", false));
    assert_eq!(1234, config.get_default_t("number", 4567));
    assert_eq!(4567, config.get_default_t("num", 4567));
}
