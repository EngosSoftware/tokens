/// Returns a collection of tokens built from the given input.
pub fn tokenize(input: &str) -> Vec<&str> {
  const QUOTE: char = '"';
  enum State {
    OutsideToken,
    InsideToken(usize),
    InsideQuote(usize),
  }
  let mut tokens: Vec<&str> = vec![];
  let mut state = State::OutsideToken;
  for (idx, ch) in input.char_indices() {
    match state {
      State::OutsideToken => match ch {
        QUOTE => state = State::InsideQuote(idx + 1),
        other if other.is_whitespace() => {}
        _ => state = State::InsideToken(idx),
      },
      State::InsideToken(start) => match ch {
        QUOTE => {
          tokens.push(&input[start..idx]);
          state = State::InsideQuote(idx + 1);
        }
        other if other.is_whitespace() => {
          tokens.push(&input[start..idx]);
          state = State::OutsideToken;
        }
        _ => {}
      },
      State::InsideQuote(start) => {
        if ch == QUOTE {
          tokens.push(&input[start..idx]);
          state = State::OutsideToken;
        }
      }
    }
  }
  match state {
    State::InsideToken(start) => tokens.push(&input[start..]),
    State::InsideQuote(start) => tokens.append(&mut tokenize(&input[start..])),
    State::OutsideToken => {}
  }
  tokens
}
