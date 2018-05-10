use std::{thread, time};
use std::time::Duration;

static TEN_MILLIS: Duration = time::Duration::from_millis(1000);

fn main() {
    let handle = thread::spawn(|| {
        println!("かえるのうたが");
        thread::sleep(TEN_MILLIS);
        println!("きこえてくるよ");
        thread::sleep(TEN_MILLIS);
        println!("ぐわ ぐわ ぐわ ぐわ");
        thread::sleep(TEN_MILLIS);
        println!("げろげろげろぐわぐわぐわ");
    });

    handle.join().unwrap();
    thread::sleep(TEN_MILLIS);
    handle.join().unwrap();
}


