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
    // スタック用配列
    let mut stack = vec![];
    // pop()は後から取り出すので，eval関数で逆順にしている
    while let Some(token) = tokens.pop() {
      if let Ok(x) = token.parse::<i32>() {
        // i32で取れたらスタックに追加
        stack.push(x);
      } else {
        // 演算子が来たらスタックから2つ取り出す
        let y = stack.pop().expect("invailed syntax");
        let x = stack.pop().expect("invailed syntax");
        // 演算子によってそれぞれの計算を行う
        let res = match token {
          "+" => x + y,
          "-" => x - y,
          "*" => x * y,
          "/" => x / y,
          "%" => x % y,
          _ => panic!("invailed token"),
        };
        // 計算結果をスタックに追加する
        stack.push(res);
      }
      // -vがある場合に途中結果を出力する
      if self.0 {
        println!("{:?} {:?}", tokens, stack);
      }
    }
    // スタックが一つの数値になったら結果を出力する
    if stack.len() == 1 {
      stack[0]
    } else {
      panic!("invailed syntax")
    }
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

// cargo buildやcargo runの場合は実行されない
#[cfg(test)]
mod tests {
  // testモジュールで有効なすべての構造体や関数を使えるようにする
  use super::*;

  #[test]
  fn test_ok() {
    let calc = RpnCulculator::new(false);
    assert_eq!(calc.eval("5"), 5);
    assert_eq!(calc.eval("50"), 50);
    assert_eq!(calc.eval("-50"), -50);
    assert_eq!(calc.eval("2 3 +"), 5);
    assert_eq!(calc.eval("2 3 *"), 6);
    assert_eq!(calc.eval("2 3 -"), -1);
    assert_eq!(calc.eval("2 3 /"), 0);
    assert_eq!(calc.eval("2 3 %"), 2);
  }

  #[test]
  #[should_panic]
  fn test_ng() {
    let calc = RpnCulculator::new(false);
    calc.eval("1 1 ^");
  }
}
