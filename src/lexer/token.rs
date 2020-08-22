pub struct Token{
  pub _let : i64,
  pub _variable:i64,
  pub _print: i64,
  pub _equal: i64,
  pub _add: i64,
  pub _sub: i64,
  pub _multi:i64,
  pub _div: i64,

  pub _end: i64,
  pub _number: i64,
}

impl Token{
  pub const fn new() -> Token{
    Token{
      _let: -1,
      _variable: -2,
      _print: -3,
      _number: -4,
      _equal: 61,
      _add: 43,
      _sub: 45,
      _multi: 37,
      _div: 47,
      _end: 59,
    }
  }
}