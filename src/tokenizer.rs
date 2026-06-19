/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: impl AsRef<str>) -> Vec<String> {
  const DOUBLE_QUOTE: char = '"';
  enum State {
    OutsideToken,
    InsideToken,
    InsideQuote,
  }
  let mut tokens = vec![];
  let mut state = State::OutsideToken;
  let mut token = String::new();
  for ch in input.as_ref().chars() {
    match state {
      State::OutsideToken => match ch {
        DOUBLE_QUOTE => {
          state = State::InsideQuote;
        }
        other if other.is_whitespace() => {}
        other => {
          token.push(other);
          state = State::InsideToken;
        }
      },
      State::InsideToken => match ch {
        DOUBLE_QUOTE => {
          tokens.push(std::mem::take(&mut token));
          state = State::InsideQuote;
        }
        other if other.is_whitespace() => {
          tokens.push(std::mem::take(&mut token));
          state = State::OutsideToken;
        }
        other => {
          token.push(other);
        }
      },
      State::InsideQuote => match ch {
        DOUBLE_QUOTE => {
          tokens.push(std::mem::take(&mut token));
          state = State::OutsideToken;
        }
        other => token.push(other),
      },
    }
  }
  if let State::InsideQuote = state {
    tokens.append(&mut tokenize(token));
  } else if !token.is_empty() {
    tokens.push(token);
  }
  tokens
}
