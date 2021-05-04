fn main() {
  // ビルドのためのインスタンスを作成し，ファイルパスを指定し，出力ファイル名を指定する
  cc::Build::new().file("./src/hello.c").compile("hello");
}
