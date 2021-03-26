use base64::decode;
use chrono::{Local, TimeZone};
use serde_json::Value;

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let token = args.next().unwrap();

    let (_, message) = split(&token);
    let (claims, _) = split(&message);

    let bytes = decode(claims).unwrap();
    let claims = serde_json::from_slice(&bytes).unwrap();

    if let Value::Object(ref map) = claims {
        if let Some(exp) = map.get("exp").and_then(|v| v.as_i64()) {
            let dt = Local.timestamp(exp, 0);
            let color = if dt > Local::now() { "32" } else { "31" };

            println!(
                "Expiration: \x1B[{}m{}\x1B[0m",
                color,
                Local.timestamp(exp, 0).to_string()
            );
        }
    }

    print_claims(claims);
}

fn split<'s>(string: &'s str) -> (&'s str, &'s str) {
    let mut parts = string.rsplitn(2, ".");
    (parts.next().unwrap(), parts.next().unwrap())
}

fn print_claims(value: Value) {
    print_value(value, 0)
}

fn print_value(value: Value, indent: u8) {
    fn padding(size: u8) -> String {
        String::from(" ").repeat(size as usize)
    }

    match value {
        Value::Null => println!("null"),
        Value::Bool(value) => println!("{}", value),
        Value::Number(value) => println!("{}", value),
        Value::String(value) => println!("{}", value),
        Value::Array(value) => {
            println!();
            for item in value {
                print!("{}- ", padding(indent));
                print_value(item, indent + 2);
            }
        }
        Value::Object(value) => {
            println!();

            let mut values = value.into_iter().collect::<Vec<_>>();
            values.sort_unstable_by_key(|(k, _)| k.clone());

            for (key, value) in values {
                print!("{}{}: ", padding(indent), key);
                print_value(value, indent + 2)
            }
        }
    }
}
