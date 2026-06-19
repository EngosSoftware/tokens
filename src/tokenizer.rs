/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: impl AsRef<str>) -> Vec<String> {
  const SINGLE_QUOTE: char = '\'';
  const DOUBLE_QUOTE: char = '"';
  enum State {
    OutsideToken,
    InsideToken,
    SingleQuote,
    DoubleQuote
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
          match ch {
            SINGLE_QUOTE => {
              token.push(SINGLE_QUOTE);
              state = State::SingleQuote;
            }
            DOUBLE_QUOTE => {
              token.push(DOUBLE_QUOTE);
              state = State::DoubleQuote;
            }
            other => {
              token.push(other);
              state = State::InsideToken;
            }
          }
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
      State::SingleQuote => {
        match ch {
          SINGLE_QUOTE => {
            let a = token[1..].to_string();
            tokens.push(a);
            token.clear();
            state = State::OutsideToken;
          }
          other => token.push(other),
        }
      }
      State::DoubleQuote => {
        match ch {
          DOUBLE_QUOTE => {
            let a = token[1..].to_string();
            tokens.push(a);
            token.clear();
            state = State::OutsideToken;
          }
          other => token.push(other),
        }
      }
    }
  }
  if token.starts_with(SINGLE_QUOTE) || token.starts_with(DOUBLE_QUOTE) {
    let mut tail_tokens = tokenize(&token[1..]);
    tokens.append(&mut tail_tokens);
  } else if !token.is_empty() {
    tokens.push(token);
  }
  tokens
}
