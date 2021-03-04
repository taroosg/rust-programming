use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
  #[error("failed to read string from {0}")]
  ReadError(String),
  #[error(transparent)]
  ParseError(#[from] std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
  let path = "number.txt";

  // read_to_stringできればエラーで早期リターン
  let num_str = std::fs::read_to_string(path).map_err(|_| MyError::ReadError(path.into()))?;
  num_str
    .trim()
    .parse::<i32>()
    // parseできたら2倍する
    .map(|t| t * 2)
    // parseできなければエラーメッセージを返す
    .map_err(MyError::from)
}

fn main() {
  match get_int_from_file() {
    // 2倍された値
    Ok(x) => println!("{}", x),
    // エラーメッセージ
    Err(e) => println!("{:?}", e),
  }
}
