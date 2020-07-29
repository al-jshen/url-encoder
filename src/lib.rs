use std::collections::HashMap;
use lazy_static::lazy_static;


lazy_static! {
    pub static ref ENCODINGS: HashMap<&'static str, &'static str> = {
        let keys: Vec<&str> = vec!["!", "#", "$", "&", "'", "(", ")", "*", "+", ",", "/", ":", ";", "=", "?", "@", "[", "]", "\n", " ", "\"", "-", ".", "<", ">", "\\", "^", "_", "`", "{", "|", "}", "~"];
        let values: Vec<&str> = vec!["%21", "%23", "%24", "%26", "%27", "%28", "%29", "%2A", "%2B", "%2C", "%2F", "%3A", "%3B", "%3D", "%3F", "%40", "%5B", "%5D", "%0A", "%20", "%22", "%2D", "%2E", "%3C", "%3E", "%5C", "%5E", "%5F", "%60", "%7B", "%7C", "%7D", "%7E"];
        let mut m = HashMap::new();
        for i in 0..keys.len() {
            m.insert(keys[i], values[i]);
        }
        m
    };
}

pub mod encode {
    use super::*;

    pub fn encode(query: &str) -> String {
        let mut new_query = query.to_owned();
        new_query = new_query.replace("%", "%25");
        for (from, to) in ENCODINGS.iter() {
            new_query = new_query.replace(from, to);
        }
        new_query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_encoding() {
        assert_eq!(ENCODINGS.get(" "), Some(&"%20"));
        assert_eq!(ENCODINGS.get("\n").unwrap(), &"%0A");
        assert_eq!(ENCODINGS.get("#").unwrap().to_owned(), "%23");
        assert_eq!(ENCODINGS.get("\\").unwrap().to_owned(), "%5C".to_owned());
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode::encode("asdf this/is a.test"), "asdf%20this%2Fis%20a%2Etest");
    }
}
