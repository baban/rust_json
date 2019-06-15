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

  impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
      Json::String(s.to_string())
    }
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
  impl_from_num_to_json!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize, f32, f64);
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

