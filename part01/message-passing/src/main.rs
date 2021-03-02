use std::sync::mpsc;
use std::thread;

fn main() {
  // // 送受信用のチャンネルを作成
  // let (tx, rx) = mpsc::channel();
  // // スレッドでは，受信した値を表示する処理を実行
  // let handle = thread::spawn(move || {
  //   let data = rx.recv().unwrap();
  //   println!("{}", data);
  // });
  // // データを送信
  // let _ = tx.send("Hello World!");
  // let _ = handle.join();

  let mut handles = vec![];
  let mut data = vec![1; 10];
  // 送信用チャンネル
  let mut snd_channels = vec![];
  // 受信用チャンネル
  let mut rcv_channels = vec![];
  for _ in 0..10 {
    // mainから各スレッドへのチャンネル
    let (snd_tx, snd_rx) = mpsc::channel();
    // 各スレッドからmainへのチャンネル
    let (rcv_tx, rcv_rx) = mpsc::channel();
    snd_channels.push(snd_tx);
    rcv_channels.push(rcv_rx);

    handles.push(thread::spawn(move || {
      let mut data = snd_rx.recv().unwrap();
      // 1を受け取ったら1足して送信
      data += 1;
      let _ = rcv_tx.send(data);
    }));
  }

  // 各スレッドにデータを送信
  for x in 0..10 {
    let _ = snd_channels[x].send(data[x]);
  }
  // 各スレッドでデータを格納
  for x in 0..10 {
    data[x] = rcv_channels[x].recv().unwrap();
  }

  for handle in handles {
    let _ = handle.join();
  }

  dbg!(data);
}
