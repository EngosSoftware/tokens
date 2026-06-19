/// Tokenizes a string into whitespace-separated words with support for quoted phrases.
pub fn tokens(input: &str) -> Vec<&str> {
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
          tokens.push(&input[start..index]);
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
