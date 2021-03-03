use std::env;

fn main() {
  // 自前で入力値を取得するパターン
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
}
