// use std::env;
use clap::{App, Arg};

fn main() {
  // // 自前で入力値を取得するパターン
  // let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);

  // clapを使用して実装するパターン
  let matches = App::new("My RPN program")
    // アプリケーションの概要
    .version("1.0.0")
    .author("Taroosg")
    .about("Super awsome sample RPN culculator")
    // index(1)で第1引数にファイルを指定
    .arg(
      Arg::new("formula_file")
        .about("Formula written in RPN")
        .value_name("FILE")
        .index(1)
        .required(false),
    )
    // -vと--verboseを指定できるように設定
    .arg(
      Arg::new("verbose")
        .about("Sets the level of verbosity")
        .short('v')
        .long("verbose")
        .required(false),
    )
    .get_matches();

  match matches.value_of("formula_file") {
    Some(file) => println!("File specified: {}", file),
    None => println!("No file specified."),
  }
  let verbose = matches.is_present("verbose");
  println!("Is verbosity specified?: {}", verbose);
}
