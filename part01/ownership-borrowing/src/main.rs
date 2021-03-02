fn main() {
  // 所有権と借用
  struct Color {
    r: i32,
    g: i32,
    b: i32,
  }
  let a = Color {
    r: 255,
    g: 255,
    b: 255,
  };
  // let b = a;  // NG：こうすると所有権が移るのでaは値を持たなくなる
  let b = &a; // OK：参照するだけなのでab両方使える
  println!("{} {} {}", b.r, b.g, b.b);
  println!("{} {} {}", a.r, a.g, a.b);

  // ライフタイム
  // xスタート
  let mut x = 5;
  // yスタート
  let y = &x;
  // yエンド
  // zスタート
  let z = &mut x;
  println!("{}", z);
  // zエンド
  println!("{}", x);
  // xエンド
}
