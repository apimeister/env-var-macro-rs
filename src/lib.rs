
#[macro_export]
macro_rules! env_var {
  //require variables
  {required $key:expr} => {
    match std::env::var($key) {
      Ok(val) => val,
      Err(_err) =>{ log::error!("missing environment variable: {}",$key);panic!();},
    }
  };
  {required $key:expr, default: $default:expr} => {
    match std::env::var($key) {
      Ok(val) => val,
      Err(_err) => $default.to_string(),
    }
  };

  //optional variables
  {optional $key:expr} => {
    match std::env::var($key) {
      Ok(val) => val.to_string(),
      Err(_err) => "".to_string(),
    }
  };
  {optional $key:expr, err_msg: $err_msg:expr} => {match_or_default($key,"")};
}

#[cfg(test)]
mod tests {
  fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
  }
  #[test]
  fn get_optional() {
    std::env::set_var("TEST", "VAL");
    let result = env_var!(optional "TEST");
    assert_eq!(result,"VAL");
  }
  #[test]
  #[should_panic]
  fn get_required_non_existent() {
    init();
    let _result = env_var!(required "NON_EXISTANT_VARIABLE");
  }
  #[test]
  fn get_required() {
    init();
    std::env::set_var("TEST1", "VAL");
    let result = env_var!(required "TEST1");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_required_or_default_value() {
    init();
    std::env::set_var("TEST2", "VAL");
    let result = env_var!(required "TEST2", default: "WHATEVER");
    assert_eq!(result,"VAL");
  }
  #[test]
  fn get_required_or_default_novalue() {
    init();
    let result = env_var!(required "TEST3", default: "WHATEVER");
    assert_eq!(result,"WHATEVER");
  }

}