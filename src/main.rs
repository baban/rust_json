use rust_json::json::*;

macro_rules! json {
  (null) => { Json::Null };
  (true) => { Json::Boolean(true) };
  (false) => { Json::Boolean(false) };
}

fn main() {
    let json = json![true];
    println!("{:?}", json);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
