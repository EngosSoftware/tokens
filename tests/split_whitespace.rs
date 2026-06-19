const EMPTY: Vec<&str> = vec![];

#[test]
fn _0001() {
  assert_eq!(EMPTY, "".split_whitespace().collect::<Vec<&str>>());
}

#[test]
fn _0002() {
  assert_eq!(vec!["a"], "  a  ".split_whitespace().collect::<Vec<&str>>());
}

#[test]
fn _0003() {
  assert_eq!(vec!["a", "b"], "  \t a   \n \r   b  \t   ".split_whitespace().collect::<Vec<&str>>());
}
