#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods() {
        let u = "blah".parse::<Ucfirst>().unwrap();
        let s = String::from("Blah");
        assert_eq!(u, Ucfirst(s.clone()));
        assert_eq!(u.to_string(), s);
        assert_eq!(format!("{u}"), s);
        assert_eq!(format!("{}", u), s);
    }

    #[test]
    fn function() {
        assert_eq!(ucfirst("blah"), "Blah");
    }
}

#[derive(Debug, PartialEq)]
pub struct UcfirstError;

#[derive(Debug, PartialEq)]
pub struct Ucfirst(String);

impl std::str::FromStr for Ucfirst {
    type Err = UcfirstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        Ok(match c.next() {
            Some(f) => Self(f.to_uppercase().collect::<String>() + c.as_str()),
            None => Self(String::new()),
        })
    }
}

impl std::fmt::Display for Ucfirst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/**
Capitalize the first letter of a [`&str`]

```
use ucfirst::ucfirst;

assert_eq!(ucfirst("apple"), "Apple");
```
*/
pub fn ucfirst(s: &str) -> String {
    s.parse::<Ucfirst>().unwrap().to_string()
}
