pub mod json {
  use std::collections::HashMap;
  pub enum Json {
      Null,
      Boolean(bool),
      Number(f64),
      String(String),
      Array(Vec<Json>),
      Object(Box<HashMap<String, Json>>)
  }
}
