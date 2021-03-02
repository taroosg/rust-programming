fn main() {
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
}
