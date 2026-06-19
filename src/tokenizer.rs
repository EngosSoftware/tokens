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
  for (index, ch) in input.char_indices() {
    match state {
      State::OutsideToken => match ch {
        QUOTE => state = State::InsideQuote(index + 1),
        other if other.is_whitespace() => {}
        _ => state = State::InsideToken(index),
      },
      State::InsideToken(start) => match ch {
        QUOTE => {
          tokens.push(&input[start..index]);
          state = State::InsideQuote(index + 1);
        }
        other if other.is_whitespace() => {
          tokens.push(&input[start..index]);
          state = State::OutsideToken;
        }
        _ => {}
      },
      State::InsideQuote(start) => {
        if ch == QUOTE {
          if index > start {
            tokens.push(&input[start..index]);
          }
          state = State::OutsideToken;
        }
      }
    }
  }
  match state {
    State::InsideToken(start) => tokens.push(&input[start..]),
    State::InsideQuote(start) => tokens.extend(input[start..].split_whitespace()),
    State::OutsideToken => {}
  }
  tokens
}
