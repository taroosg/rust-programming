fn main() {
  // let result: Result<i32, String> = Ok(200);
  let result: Result<i32, String> = Err("error".to_string());
  // `unwrap_or()`を使うとErrなら-1にする実装
  println!("code:{}", result.unwrap_or(-1));

  let v1 = vec![0, 1, 2, 3, 4];
  // アクセスできない要素なら-1を返す実装
  println!("{:?}", v1.get(8).unwrap_or(&-1));

  // ifは式
  let number = -10;
  let if_result = if number < 0 { number } else { -number };
  println!("{:?}", if_result);

  // loopも式
  let mut loop_count = 1;
  let loop_result = loop {
    println!("count: {:?}", loop_count);
    loop_count += 1;
    if loop_count == 10 {
      break loop_count;
    }
  };
  println!("{:?}", loop_result);

  // 構造体とimpl
  struct Person {
    name: String,
    age: i32,
  }

  impl Person {
    // コンストラクタ
    fn new(name: &str, age: i32) -> Person {
      Person {
        name: name.to_string(),
        age: age,
      }
    }
    // 返り値をSelfにすることでメソッドチェーンを組める
    fn say_name(&self) -> &Self {
      println!("{}", self.name);
      self
    }
    fn say_age(&self) -> &Self {
      println!("{}", self.age);
      self
    }
    // 後に実装する予定のものは`todo!()`マクロで記述すると便利
    fn attack(&self, target: Person) -> Person {
      todo!()
    }
  }
  // 実行
  let p = Person::new("taro", 33);
  p.say_name().say_age();

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
