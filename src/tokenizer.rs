/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: impl AsRef<str>) -> Vec<String> {
  const DOUBLE_QUOTE: char = '"';
  enum State {
    OutsideToken,
    InsideToken(usize),
    InsideQuote(usize),
  }
  let mut tokens = vec![];
  let mut state = State::OutsideToken;
  let input = input.as_ref();
  for (idx, ch) in input.char_indices() {
    match state {
      State::OutsideToken => match ch {
        DOUBLE_QUOTE => state = State::InsideQuote(idx + 1),
        other if other.is_whitespace() => {}
        _ => state = State::InsideToken(idx),
      },
      State::InsideToken(start) => match ch {
        DOUBLE_QUOTE => {
          tokens.push(input[start..idx].to_string());
          state = State::InsideQuote(idx + 1);
        }
        other if other.is_whitespace() => {
          tokens.push(input[start..idx].to_string());
          state = State::OutsideToken;
        }
        _ => {}
      },
      State::InsideQuote(start) => {
        if ch == DOUBLE_QUOTE {
          tokens.push(input[start..idx].to_string());
          state = State::OutsideToken;
        }
      }
    }
  }
  match state {
    State::InsideToken(start) => tokens.push(input[start..].to_string()),
    State::InsideQuote(start) => tokens.append(&mut tokenize(&input[start..])),
    State::OutsideToken => {}
  }
  tokens
}
