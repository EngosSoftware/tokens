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

#[test]
fn _0013() {
  assert_eq!(vec!["alpha", "beta gamma", "delta"], tokenize(r#"    alpha "beta gamma" delta  "#));
}

#[test]
fn _0014() {
  assert_eq!(vec!["alpha", "beta gamma", "delta"], tokenize("    alpha 'beta gamma' delta  "));
}

#[test]
fn _0015() {
  assert_eq!(vec!["alpha", "\"beta", "gamma", "delta"], tokenize(r#"    alpha "beta   gamma  delta  "#));
}

#[test]
fn _0016() {
  assert_eq!(vec!["alpha", "'beta", "gamma", "delta"], tokenize("    alpha 'beta gamma delta  "));
}

#[test]
fn _0017() {
  assert_eq!(vec!["alpha", "beta", "gamma", "delta"], tokenize(r#"alpha"beta"gamma delta"#));
}

#[test]
fn _0018() {
  assert_eq!(vec!["alpha", "beta", "gamma", "delta"], tokenize("alpha'beta'gamma delta"));
}

#[test]
fn _0019() {
  assert_eq!(vec!["alpha"], tokenize(r#"alpha""#));
}

#[test]
fn _0020() {
  assert_eq!(vec!["alpha"], tokenize("alpha'"));
}

#[test]
fn _0021() {
  assert_eq!(vec!["alpha", "beta", "gamma", "\"delta"], tokenize(r#"alpha"beta"gamma"delta"#));
}

#[test]
fn _0022() {
  assert_eq!(vec!["alpha", "beta", "gamma", "'delta"], tokenize("alpha'beta'gamma'delta"));
}
