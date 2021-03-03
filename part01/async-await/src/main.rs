use async_trait::async_trait;
use futures::executor;

// traitではasyncできないが，async_traitを使うとできる
#[async_trait]
trait AsyncTrait {
  async fn f(&self);
}
struct Runner {}
#[async_trait]
impl AsyncTrait for Runner {
  async fn f(&self) {
    println!("Hello Async Fn!");
  }
}

async fn async_add(left: i32, right: i32) -> i32 {
  left + right
}

async fn something_greet_async_function() -> i32 {
  // 順番にawaitで処理
  let ans1 = async_add(2, 3).await;
  let ans2 = async_add(3, 4).await;
  let ans3 = async_add(4, 5).await;
  // 全部揃ったら加算
  let result = ans1 + ans2 + ans3;
  println!("{}", result);
  result
}

// main関数はasyncできない
// fn main() {
//   // block_on()内に記述してasync-awaitする
//   executor::block_on(something_greet_async_function());
// }

// async-stdを使うとmain関数をasyncできる
#[async_std::main]
async fn main() {
  // hahaha!!!
  let ans1 = async_add(1, 2).await;
  let ans2 = async_add(2, 3).await;
  let ans3 = async_add(3, 4).await;
  let result = ans1 + ans2 + ans3;
  println!("{}", result);
}
