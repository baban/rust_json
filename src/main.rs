use rust_json::json::*;

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
}

fn main() {
    let json = json![true];
    println!("{:?}", json);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn null_works() {
        let json = json![null];
        let result = Json::Null;
        assert_eq!(json, result);
    }

    #[test]
    fn true_works() {
        let json = json![true];
        let result = Json::Boolean(true);
        assert_eq!(json, result);
    }

    #[test]
    fn false_works() {
        let json = json![false];
        let result = Json::Boolean(false);
        assert_eq!(json, result);
    }

    #[test]
    fn array_works() {
        let json = json!([true, false]);
        let result = Json::Array(vec![Json::Boolean(true), Json::Boolean(false)]);
        assert_eq!(json, result);
    }

    #[test]
    fn one_hash_works() {
        let json = json!({ "a": true });

        let mut map = HashMap::new();
        map.insert("a".to_string(), Json::Boolean(true));

        let result = Json::Object(Box::new(map));
        assert_eq!(json, result);
    }
}
