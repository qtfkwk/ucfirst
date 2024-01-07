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
