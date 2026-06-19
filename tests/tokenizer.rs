use tokens::tokenize;

const EMPTY: Vec<String> = vec![];

#[test]
fn _0001() {
  assert_eq!(EMPTY, tokenize(""));
}

#[test]
fn _0002() {
  assert_eq!(EMPTY, tokenize("    \r   \n     \t   "));
}

#[test]
fn _0003() {
  assert_eq!(vec!["a"], tokenize("a"));
}

#[test]
fn _0004() {
  assert_eq!(vec!["a"], tokenize("   a"));
}

#[test]
fn _0005() {
  assert_eq!(vec!["a"], tokenize("a     "));
}

#[test]
fn _0006() {
  assert_eq!(vec!["a"], tokenize("    a     "));
}

#[test]
fn _0007() {
  assert_eq!(vec!["a", "b", "c"], tokenize("    a   b\nc  "));
}

#[test]
fn _0008() {
  assert_eq!(vec!["alpha"], tokenize("alpha"));
}

#[test]
fn _0009() {
  assert_eq!(vec!["alpha"], tokenize("   alpha"));
}

#[test]
fn _0010() {
  assert_eq!(vec!["alpha"], tokenize("alpha     "));
}

#[test]
fn _0011() {
  assert_eq!(vec!["alpha"], tokenize("    alpha     "));
}

#[test]
fn _0012() {
  assert_eq!(vec!["alpha", "beta", "gamma"], tokenize("    alpha   beta\ngamma  "));
}
