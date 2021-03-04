fn get_int_from_file() -> Result<i32, String> {
  let path = "number.txt";

  // read_to_stringできればエラーで早期リターン
  let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
  num_str
    .trim()
    .parse::<i32>()
    // parseできたら2倍する
    .map(|t| t * 2)
    // parseできなければエラーメッセージを返す
    .map_err(|e| e.to_string())
}

fn main() {
  match get_int_from_file() {
    // 2倍された値
    Ok(num) => println!("{}", num),
    // エラーメッセージ
    Err(e) => println!("{}", e),
  }
  // println!("{}", get_int_from_file());
}
