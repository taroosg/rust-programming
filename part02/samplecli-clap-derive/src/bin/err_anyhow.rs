use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
  let path = "number.txt";

  // read_to_stringできればエラーで早期リターン
  let num_str = std::fs::read_to_string(path)
    .with_context(|| format!("failed to read string from {}", path))?;
  num_str
    .trim()
    .parse::<i32>()
    // parseできたら2倍する
    .map(|t| t * 2)
    // parseできなければエラーメッセージを返す
    .context("failed to parse string")
}

fn main() {
  match get_int_from_file() {
    // 2倍された値
    Ok(x) => println!("{}", x),
    // エラーメッセージ
    Err(e) => println!("{:?}", e.source()),
  }
}
