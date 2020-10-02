pub struct Token{
  pub _let : i64,
  pub _variable:i64,
  pub _number: i64,
  pub _string: i64,
  pub _if: i64,
  pub _else: i64,
  pub _for: i64,
  pub _fn: i64,
  pub _return: i64,
  pub _equal: i64,
  pub _greater: i64,
  pub _less: i64,
  pub _add: i64,
  pub _sub: i64,
  pub _multi:i64,
  pub _div: i64,
  pub _exclamation: i64,
  pub _paren_left:i64,
  pub _paren_right: i64,
  pub _braces_left:i64,
  pub _braces_right: i64,
  pub _square_brackets_left: i64,
  pub _square_brackets_right: i64,
  pub _colon: i64,
  pub _comma: i64,
  pub _end: i64,
}

impl Token{
  pub const fn new() -> Token{
    Token{
      _let: -1,
      _variable: -2,
      _number: -3,
      _string: -4,
      _if: -5,
      _else:-6,
      _for:-7,
      _fn: -8,
      _return: -9,
      _equal: 61,
      _greater:62,
      _less:60,
      _exclamation:33,
      _add: 43,
      _sub: 45,
      _multi: 47,
      _div: 42,
      _end: 59,
      _square_brackets_left:91,
      _square_brackets_right:93,
      _paren_left: 40,
      _paren_right:41,
      _braces_left:123,
      _braces_right:125,
      _colon:58,
      _comma:44,
    }
  }
}