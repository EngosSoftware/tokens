/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: impl AsRef<str>) -> Vec<String> {
  enum State {
    OutsideToken,
    InsideToken
  }
  let mut tokens = vec![];
  let mut state = State::OutsideToken;
  let mut token = String::new();
  for ch in input.as_ref().chars() {
    match state {
      State::OutsideToken => {
        if ch.is_whitespace() {
          state = State::OutsideToken;
        }
        else {
          token.push(ch);
          state = State::InsideToken;
        }
      }
      State::InsideToken => {
        if ch.is_whitespace() {
          tokens.push(std::mem::take(&mut token));
          state = State::OutsideToken;
        } else {
          token.push(ch);
        }
      }
    }
  }
  if !token.is_empty() {
    tokens.push(token);
  }
  tokens
}
