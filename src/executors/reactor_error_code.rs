pub enum ReactorErrorCode {
  OrderNotFillable, 
  Unknown,
}

// implements the From trait for the ReactorErrorCode enum to convert it to a string
impl From<String> for ReactorErrorCode {
  fn from(s: String) -> Self {
    match s.as_str() {
      "0xc6035520" => ReactorErrorCode::OrderNotFillable,
      _ => ReactorErrorCode::Unknown,
    }
  }
}

impl std::fmt::Display for ReactorErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReactorErrorCode::OrderNotFillable => "0xc6035520",
            ReactorErrorCode::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    } 
}
