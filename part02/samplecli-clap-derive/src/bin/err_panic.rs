use std::fmt;
// 独自のエラー型
enum MyError {
  Io(std::io::Error),
  Num(std::num::ParseIntError),
}

impl fmt::Display for MyError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
      MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
    }
  }
}

fn get_int_from_file() -> Result<i32, MyError> {
  let path = "number.txt";

  // read_to_stringできればエラーで早期リターン
  let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;
  num_str
    .trim()
    .parse::<i32>()
    // parseできたら2倍する
    .map(|t| t * 2)
    // parseできなければエラーメッセージを返す
    .map_err(|e| MyError::Num(e))
}

fn main() {
  match get_int_from_file() {
    // 2倍された値
    Ok(x) => println!("{}", x),
    // エラーメッセージ
    // Err(e) => match e {
    //   MyError::Io(cause) => println!("I/O Error: {}", cause),
    //   MyError::Num(cause) => println!("Parse Error: {}", cause),
    // },
    Err(e) => println!("{}", e),
  }
  // println!("{}", get_int_from_file());
}
