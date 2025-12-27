pub mod constants;

pub fn to_camel_case(s: &str) -> String {
    let mut parts = s.split("_");
    let first = parts.next().unwrap_or("").to_lowercase();
    let rest = parts.into_iter().map(|part| {
        let mut chars = part.chars();
        match chars.next() {
            Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    });
    first + rest.collect::<String>().as_str()
}
