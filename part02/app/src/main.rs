fn main() {
  println!("Hello, world!");
  library::serde();
  library::special();
  // cargo.tomlに書いていないので実行できない
  // library::parallel();
}
