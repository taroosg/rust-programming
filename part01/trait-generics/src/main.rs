fn main() {
  // trait関連
  // 異なる構造体で共通して使いそうなものをtraitとして定義
  trait Tweet {
    // tweetはimplとして実装する
    fn tweet(&self);
    fn tweet_twice(&self) {
      self.tweet();
      self.tweet();
    }
    fn shout(&self) {
      println!("Fooooooo!");
    }
  }
  struct Dove;
  struct Duck;
  // ここでtweetを実装
  impl Tweet for Dove {
    fn tweet(&self) {
      println!("Coo!");
    }
  }
  impl Tweet for Duck {
    fn tweet(&self) {
      println!("Quack!");
    }
  }
  let dove = Dove {};
  dove.tweet();
  dove.tweet_twice();
  dove.shout();
  let duck = Duck {};
  // 型が異なるけど同一のtraitを持っているのでBoxを使ってVecをつくれる．
  // 同じメソッドを実行可能
  let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
  for bird in bird_vec {
    bird.tweet();
  }

  // ジェネリクス
  // 型を指定せずに，どんな型でも同じ処理をできるようにする
  fn make_tupple<T, S>(t: T, s: S) -> (T, S) {
    (t, s)
  }
  println!("{:?}", make_tupple(1, 2));
  println!("{:?}", make_tupple("hello", "world"));
  println!("{:?}", make_tupple(vec![1, 2, 3], vec![4, 5, 6]));
}
