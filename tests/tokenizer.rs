use tokens::tokens;

const EMPTY: Vec<&str> = vec![];

#[test]
fn _0001() {
  assert_eq!(EMPTY, tokens(""));
}

#[test]
fn _0002() {
  assert_eq!(EMPTY, tokens("    \r   \n     \t   "));
}

#[test]
fn _0003() {
  assert_eq!(vec!["a"], tokens("a"));
}

#[test]
fn _0004() {
  assert_eq!(vec!["a"], tokens("   a"));
}

#[test]
fn _0005() {
  assert_eq!(vec!["a"], tokens("a     "));
}

#[test]
fn _0006() {
  assert_eq!(vec!["a"], tokens("    a     "));
}

#[test]
fn _0007() {
  assert_eq!(vec!["a", "b", "c"], tokens("    a   b\nc  "));
}

#[test]
fn _0008() {
  assert_eq!(vec!["alpha"], tokens("alpha"));
}

#[test]
fn _0009() {
  assert_eq!(vec!["alpha"], tokens("   alpha"));
}

#[test]
fn _0010() {
  assert_eq!(vec!["alpha"], tokens("alpha     "));
}

#[test]
fn _0011() {
  assert_eq!(vec!["alpha"], tokens("    alpha     "));
}

#[test]
fn _0012() {
  assert_eq!(vec!["alpha", "beta", "gamma"], tokens("    alpha   beta\ngamma  "));
}

#[test]
fn _0013() {
  assert_eq!(vec!["alpha", "beta gamma", "delta"], tokens(r#"    alpha "beta gamma" delta  "#));
}

#[test]
fn _0014() {
  assert_eq!(vec!["alpha", "'beta", "gamma's", "delta"], tokens("    alpha 'beta gamma's delta  "));
}

#[test]
fn _0015() {
  assert_eq!(vec!["alpha", "beta", "gamma", "delta"], tokens(r#"    alpha "beta   gamma  delta  "#));
}

#[test]
fn _0016() {
  assert_eq!(vec!["alpha", "'beta", "gamma", "delta"], tokens("    alpha 'beta gamma delta  "));
}

#[test]
fn _0017() {
  assert_eq!(vec!["alpha", "beta", "gamma", "delta"], tokens(r#"alpha"beta"gamma delta"#));
}

#[test]
fn _0018() {
  assert_eq!(vec!["alpha'beta'gamma", "delta"], tokens("alpha'beta'gamma delta"));
}

#[test]
fn _0019() {
  assert_eq!(vec!["alpha"], tokens(r#"alpha""#));
}

#[test]
fn _0020() {
  assert_eq!(vec!["alpha'"], tokens("alpha'"));
}

#[test]
fn _0021() {
  assert_eq!(vec!["alpha", "beta", "gamma", "delta"], tokens(r#"alpha"beta"gamma"delta"#));
}

#[test]
fn _0022() {
  assert_eq!(vec!["alpha", "beta", "gamma", "delta"], tokens(r#" "alpha" "beta" "gamma" "delta" "#));
}

#[test]
fn _0023() {
  assert_eq!(vec!["alpha'beta'gamma'delta"], tokens("alpha'beta'gamma'delta"));
}

#[test]
fn _0024() {
  assert_eq!(vec!["a"], tokens(r#""a"#));
  assert_eq!(vec!["", "a"], tokens(r#"""a"#));
  assert_eq!(vec!["", "a"], tokens(r#""""a"#));
  assert_eq!(vec!["", "", "a"], tokens(r#"""""a"#));
}

#[test]
fn _0025() {
  assert_eq!(EMPTY, tokens(r#""#));
  assert_eq!(EMPTY, tokens(r#"""#));
  assert_eq!(vec![""], tokens(r#""""#));
  assert_eq!(vec![""], tokens(r#"""""#));
  assert_eq!(vec!["", ""], tokens(r#""""""#));
  assert_eq!(vec!["", ""], tokens(r#"""""""#));
  assert_eq!(vec!["", "", ""], tokens(r#""""""""#));
}
