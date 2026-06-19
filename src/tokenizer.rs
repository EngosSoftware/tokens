/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: impl AsRef<str>) -> Vec<String> {
  enum State {
    Start,
    Whitespace,
    Token
  }
  let mut tokens = vec![];
  let mut state = State::Start;
  let mut token = String::new();
  for ch in input.as_ref().chars() {
    match state {
      State::Start => {
        if ch.is_whitespace() {
          state = State::Whitespace;
        } else {
          token.push(ch);
          state = State::Token;
        }
      }
      State::Whitespace => {
        if !ch.is_whitespace() {
          token.push(ch);
          state = State::Token;
        }
      }
      State::Token => {
        if ch.is_whitespace() {
          tokens.push(token.clone());
          token.clear();
          state = State::Whitespace;
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
