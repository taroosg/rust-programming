use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  // スレッドのハンドルが返ってくる
  // let handle = thread::spawn(|| {
  //   println!("Hello World!");
  // });
  // joinを呼ぶことでスレッドの終了を待つ
  // dbg!(handle.join());

  // 複数スレッド
  let mut handles = vec![];
  // Arcは複数スレッドで参照を更新できるようにする
  // ただしRcよりコストが重いので，単スレッドではRcを使用する
  // Mutexはmut的な感じ
  let data = Arc::new(Mutex::new(vec![1; 10]));
  for x in 0..10 {
    let data_ref = data.clone();
    // xの生存期間がスレッドよりも長くなる可能性があるのでmoveでxの所有権をスレッドに移す必要がある
    handles.push(thread::spawn(move || {
      // println!("Hello World! {}", x);
      // lockを使うと他のスレッドはアクセスできなくなる
      // 一つのスレッドで処理が完了して参照を破棄すると次のスレッドが値を参照する
      let mut data = data_ref.lock().unwrap();
      data[x] += 1;
    }));
  }
  for handle in handles {
    // 値を使用しない場合は`_`にする
    let _ = handle.join();
  }
  dbg!(data);
}
