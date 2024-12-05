#![doc = include_str!("../README.md")]

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

impl std::cmp::PartialEq<String> for Ucfirst {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialEq<&str> for Ucfirst {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

//--------------------------------------------------------------------------------------------------

pub trait UcfirstTrait {
    fn ucfirst(&self) -> String;
}

impl UcfirstTrait for String {
    fn ucfirst(&self) -> String {
        ucfirst(self)
    }
}

impl UcfirstTrait for &str {
    fn ucfirst(&self) -> String {
        ucfirst(self)
    }
}

//--------------------------------------------------------------------------------------------------

/**
Capitalize the first letter of a [`&str`]

```
use ucfirst::ucfirst;

assert_eq!(ucfirst("apple"), "Apple");
```
*/
pub fn ucfirst(s: &str) -> String {
    s.parse::<Ucfirst>().unwrap().0
}
