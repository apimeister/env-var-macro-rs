//! module for accessing environment variables through macros.
#![warn(missing_docs)]

/// env_var macro
/// 
/// ## Examples
///
/// ```ignore
/// #[macro_use]
/// extern crate env_var;
///
/// fn main() {
///   // retrieving a optional value
///   // var1 either contains the value or an empty string
///   let var1 = env_var!(optional "TEST_OPT_1");
///
///   // retrieving a optional value with default
///   // var1 either contains the value or "default1"
///   let var1 = env_var!(optional "TEST_OPT_1", default: "default1");
///
///   // retrieving a optional value with default
///   // if not found, the message will be logged as info message
///   // var1 either contains the value or "default1"
///   let var1 = env_var!(optional "TEST_OPT_1", default: "default1", msg: "using default");
///
///   // retrieving a required value
///   // var1 either contains the value or the programm panics
///   let var1 = env_var!(required "TEST_OPT_1");
///
///   // retrieving a required value with default
///   // var1 either contains the value or "default1"
///   let var1 = env_var!(required "TEST_OPT_1", default: "default1");
///
///   // retrieving a required value with default
///   // if not found, the message will be logged as warn message
///   // var1 either contains the value or "default1"
///   let var1 = env_var!(required "TEST_OPT_1", default: "default1", msg: "using default");
/// }
/// ```
#[macro_export]
macro_rules! env_var {
  //require variables
  {required $key:expr} => {
    match std::env::var($key) {
      Ok(val) => val,
      Err(_err) =>{
        log::error!("missing environment variable: {}",$key);
        panic!("missing environment variable");
      },
    }
  };
  {required $key:expr, default: $default:expr} => {
    match std::env::var($key) {
      Ok(val) => val,
      Err(_err) => $default.to_string(),
    }
  };
  {required $key:expr, msg: $msg:expr} => {
    match std::env::var($key) {
      Ok(val) => val,
      Err(_err) =>{
        log::error!("{}",$msg);
        panic!("missing environment variable");
      },
    }
  };
  {required $key:expr, default: $default:expr, msg: $msg:expr} => {
    match std::env::var($key) {
      Ok(val) => val,
      Err(_err) =>{
        log::warn!("{}",$msg);
        $default.to_string()
      },
    }
  };

  //optional variables
  {optional $key:expr} => {
    match std::env::var($key) {
      Ok(val) => val.to_string(),
      Err(_err) => "".to_string(),
    }
  };
  {optional $key:expr, default: $default:expr} => {
    match std::env::var($key) {
      Ok(val) => val.to_string(),
      Err(_err) => {
        $default.to_string()
      },
    }
  };
  {optional $key:expr, msg: $msg:expr} => {
    match std::env::var($key) {
      Ok(val) => val.to_string(),
      Err(_err) => {
        log::warn!("{}",$msg);
        "".to_string()
      },
    }
  };
  {optional $key:expr, default: $default:expr, msg: $msg:expr} => {
    match std::env::var($key) {
      Ok(val) => val.to_string(),
      Err(_err) => {
        log::info!("{}",$msg);
        $default.to_string()
      },
    }
  };
  {optional $key:expr, msg: $msg:expr, default: $default:expr} => {
    match std::env::var($key) {
      Ok(val) => val.to_string(),
      Err(_err) => {
        log::info!("{}",$msg);
        $default.to_string()
      },
    }
  };
}

#[cfg(test)]
mod tests {
  #[test]
  fn get_optional_found() {
    std::env::set_var("TEST_OPT_1", "VAL");
    let result: String = env_var!(optional "TEST_OPT_1");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_optional_notfound() {
    let result: String = env_var!(optional "TEST_OPT_2");
    assert_eq!(result,"");
  }
  #[test]
  fn get_optional_msg_found() {
    std::env::set_var("TEST_OPT_3", "VAL");
    let result: String = env_var!(optional "TEST_OPT_3", msg: "var not found");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_optional_msg_notfound() {
    std::env::set_var("TEST_OPT_4", "VAL");
    let result: String = env_var!(optional "TEST_OPT_4", msg: "var not found");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_optional_default_found() {
    std::env::set_var("TEST_OPT_5", "VAL");
    let result: String = env_var!(optional "TEST_OPT_5", default: "whatever");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_optional_default_notfound() {
    let result: String = env_var!(optional "TEST_OPT_6", default: "whatever");
    assert_eq!(result,"whatever");
  }
  #[test]
  fn get_optional_default_msg_found() {
    std::env::set_var("TEST_OPT_7", "VAL");
    let result: String = env_var!(optional "TEST_OPT_7", default: "whatever", msg: "using default value");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_optional_default_msg_notfound() {
    let result: String = env_var!(optional "TEST_OPT_8", default: "whatever", msg: "using default value");
    assert_eq!(result,"whatever");
  }
  #[test]
  fn get_optional_msg_default_found() {
    std::env::set_var("TEST_OPT_9", "VAL");
    let result: String = env_var!(optional "TEST_OPT_9", msg: "using default value", default: "whatever");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_optional_msg_default_notfound() {
    let result: String = env_var!(optional "TEST_OPT_10", default: "whatever", msg: "using default value");
    assert_eq!(result,"whatever");
  }
  #[test]
  #[should_panic]
  fn get_required_notfound() {
    let _result: String = env_var!(required "NON_EXISTANT_VARIABLE");
  }
  #[test]
  fn get_required_found() {
    std::env::set_var("TEST_REQ_1", "VAL");
    let result: String = env_var!(required "TEST_REQ_1");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_required_or_default_found() {
    std::env::set_var("TEST_REQ_2", "VAL");
    let result: String = env_var!(required "TEST_REQ_2", default: "WHATEVER");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_required_or_default_notfound() {
    let result: String = env_var!(required "TEST_REQ_3", default: "WHATEVER");
    assert_eq!(result,"WHATEVER");
  }

  #[test]
  fn get_required_msg_found() {
    std::env::set_var("TEST_REQ_4", "VAL");
    let result: String = env_var!(required "TEST_REQ_4", default: "value not found");
    assert_eq!(result,"VAL");
  }
  #[test]
  #[should_panic]
  fn get_required_msg_notfound() {
    let _result: String = env_var!(required "TEST_REQ_5", msg: "value not found");
  }
}