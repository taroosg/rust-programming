use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);

impl Future for CountDown {
  type Output = String;
  fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
    if self.0 == 0 {
      Poll::Ready("Zero".to_string())
    } else {
      // カウントを1減らす
      println!("{}", self.0);
      self.0 -= 1;
      // 処理が終わったら他の処理を待つ？
      cx.waker().wake_by_ref();
      Poll::Pending
    }
  }
}
fn main() {
  // 別々のカウントダウンを作成
  let countdown_future1 = CountDown(10);
  let countdown_future2 = CountDown(20);
  let cd_set = join_all(vec![countdown_future1, countdown_future2]);
  // 両方終わるまで処理をブロック
  let res = executor::block_on(cd_set);
  for (i, s) in res.iter().enumerate() {
    println!("{} {}", i, s);
  }
}
