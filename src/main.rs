use rust_json::*;
use rust_json::json::*;

fn main() {
    let json = json![true];
    // let result = Json::Null;
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

    #[test]
    fn hash_works() {
        let json = json!({ "a": true, "b": false });

        let mut map = HashMap::new();
        map.insert("a".to_string(), Json::Boolean(true));
        map.insert("b".to_string(), Json::Boolean(false));

        let result = Json::Object(Box::new(map));
        assert_eq!(json, result);
    }

    #[test]
    fn i8_works() {
        let n: i8 = 1;
        let json = json!(n);
        let result = Json::Number(1.0);
        assert_eq!(json, result);
    }

    #[test]
    fn u8_works() {
        let n: u8 = 1;
        let json = json!(n);
        let result = Json::Number(1.0);
        assert_eq!(json, result);
    }

    #[test]
    fn u16_works() {
        let n: u16 = 1;
        let json = json!(n);
        let result = Json::Number(1.0);
        assert_eq!(json, result);
    }

    #[test]
    fn i16_works() {
        let n: i16 = 1;
        let json = json!(n);
        let result = Json::Number(1.0);
        assert_eq!(json, result);
    }

    #[test]
    fn i32_works() {
        let n: i32 = 1;
        let json = json!(n);
        let result = Json::Number(1.0);
        assert_eq!(json, result);
    }

    #[test]
    fn f64_works() {
        let n: f64 = 1.0;
        let json = json!(n);
        let result = Json::Number(1.0);
        assert_eq!(json, result);
    }
}
