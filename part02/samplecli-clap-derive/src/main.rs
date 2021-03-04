use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

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

// 計算用の構造体
struct RpnCulculator(bool);

impl RpnCulculator {
  pub fn new(verbose: bool) -> Self {
    Self(verbose)
  }

  pub fn eval(&self, formula: &str) -> i32 {
    // 配列にして逆順にする
    let mut tokens = formula.split_ascii_whitespace().rev().collect::<Vec<_>>();
    // dbg!(&tokens);
    self.eval_inner(&mut tokens)
  }

  fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
    0
  }
}

fn main() {
  // optsに入力した引数が格納される
  let opts = Opts::parse();
  // println!("File specified: {}", opts.formula_file);
  // // 4-1項のパターンマッチング
  // match opts.formula_file {
  //   Some(file) => println!("File specified: {}", file),
  //   None => println!("No file specified."),
  // }
  // println!("Is verbosity specified?: {}", opts.verbose);

  // ファイルが指定された場合
  if let Some(path) = opts.formula_file {
    // ファイル開く
    let f = File::open(path).unwrap();
    // ファイルの中身をまとめて返してくれる
    let reader = BufReader::new(f);
    // 1行毎にforで表示
    // for line in reader.lines() {
    //   let line = line.unwrap();
    //   println!("{}", line)
    // }
    run(reader, opts.verbose);
  } else {
    // ファイルが指定されなかった場合
    // println!("No file specified");
    let stdin = stdin();
    let reader = stdin.lock();
    // このreaderはStdinLick型
    run(reader, opts.verbose);
  }
}

// StdinLick型とBufReader<File>型はどちらもBufReadトレイトを実装しているため，BufReadトレイトを実装している型（Rとする）なら受け取れるようにする
fn run<R: BufRead>(reader: R, verbose: bool) {
  let calc = RpnCulculator::new(verbose);
  for line in reader.lines() {
    let line = line.unwrap();
    let answer = calc.eval(&line);
    println!("{}", answer);
  }
}