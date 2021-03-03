use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
  name = "My RPN program",
  version = "1.0.0",
  author = "Taroosg",
  about = "Super awsome sample RPN culculator"
)]

// 自作の構造体を定義し，コマンドライン引数の中身を決める
// 自動的に引数を設定した型に変換して格納する
struct Opts {
  #[clap(short, long)]
  verbose: bool,
  #[clap(name = "FILE")]
  // Optionにすることで，任意の引数である旨を表せる
  formula_file: Option<String>,
  // デフォルト値の設定
  // #[clap(name = "FILE", default_value = "default.txt")]
  // formula_file: String,
}

fn main() {
  // optsに入力した引数が格納される
  let opts = Opts::parse();
  // println!("File specified: {}", opts.formula_file);
  match opts.formula_file {
    Some(file) => println!("File specified: {}", file),
    None => println!("No file specified."),
  }
  println!("Is verbosity specified?: {}", opts.verbose);
}
// 次は154ページから
