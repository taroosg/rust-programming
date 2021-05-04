// c_hello()をRust内で使用する宣言
extern "C" {
  fn c_hello();
  fn c_fib(n: u32) -> u32;
}

fn main() {
  println!("Hello, world from Rust\n");
  // unsafeはRustのコンパイラで安全性をチェックできる領域外のコードを参照することを示す
  unsafe {
    c_hello();
    println!("{}", c_fib(45));
  }
}
