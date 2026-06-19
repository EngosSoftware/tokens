/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: impl AsRef<str>) -> Vec<String> {
  const SINGLE_QUOTE: char = '\'';
  const DOUBLE_QUOTE: char = '"';
  enum State {
    OutsideToken,
    InsideToken,
    InsideQuote(char),
  }
  let mut tokens = vec![];
  let mut state = State::OutsideToken;
  let mut token = String::new();
  for ch in input.as_ref().chars() {
    match state {
      State::OutsideToken => match ch {
        SINGLE_QUOTE | DOUBLE_QUOTE => {
          state = State::InsideQuote(ch);
        }
        other if other.is_whitespace() => {}
        other => {
          token.push(other);
          state = State::InsideToken;
        }
      },
      State::InsideToken => match ch {
        SINGLE_QUOTE | DOUBLE_QUOTE => {
          tokens.push(std::mem::take(&mut token));
          state = State::InsideQuote(ch);
        }
        other if other.is_whitespace() => {
          tokens.push(std::mem::take(&mut token));
          state = State::OutsideToken;
        }
        other => {
          token.push(other);
        }
      },
      State::InsideQuote(quote) => {
        if ch == quote {
          tokens.push(std::mem::take(&mut token));
          state = State::OutsideToken;
        } else {
          token.push(ch);
        }
      }
    }
  }
  if let State::InsideQuote(quote) = state {
    let mut tail_tokens = tokenize(token);
    if let Some(first) = tail_tokens.first_mut() {
      first.insert(0, quote);
    }
    tokens.append(&mut tail_tokens);
  } else if !token.is_empty() {
    tokens.push(token);
  }
  tokens
}
