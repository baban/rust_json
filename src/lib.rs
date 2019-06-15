pub mod json {
  use std::collections::HashMap;
  
  #[derive(Clone, PartialEq, Debug)]
  pub enum Json {
      Null,
      Boolean(bool),
      Number(f64),
      String(String),
      Array(Vec<Json>),
      Object(Box<HashMap<String, Json>>)
  }

  macro_rules! impl_from_num_to_json {
    ( $( $t:ident ),* ) => {
      $(
        impl From<$t> for Json {
          fn from(n: $t) -> Json {
            Json::Number(n as f64)
          }
        }
      )*
    }
  }
  impl_from_num_to_json!(i32, f64);
}

#[macro_export]
macro_rules! json {
  (null) => { Json::Null };
  (true) => { Json::Boolean(true) };
  (false) => { Json::Boolean(false) };
  ([ $($element:tt),* ]) => {
      Json::Array(vec![ $( json!($element) ), * ])
  };
  ({ $($key:tt : $value:tt),* }) => {
      Json::Object(
          Box::new(
              vec![
                  $( ($key.to_string(), json!($value) ) ), *
              ].into_iter().collect()
          )
      )
  };
  ($other:tt) => {
      Json::from($other)
  }
}

