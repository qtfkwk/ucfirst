use ucfirst::*;

#[test]
fn ucfirst_struct() {
    let u = "blah".parse::<Ucfirst>().unwrap();
    let s = String::from("Blah");
    assert_eq!(u, s);
    assert_eq!(u.to_string(), s);
    assert_eq!(format!("{u}"), s);
    assert_eq!(format!("{}", u), s);
}

#[test]
fn ucfirst_function() {
    assert_eq!(ucfirst("blah"), "Blah");
}

#[test]
fn ucfirst_trait() {
    assert_eq!("blah".ucfirst(), "Blah");
}
