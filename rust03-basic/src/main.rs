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
  }
  // 実行
  let p = Person::new("taro", 33);
  p.say_name().say_age();
}
