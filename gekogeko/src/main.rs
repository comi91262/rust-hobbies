use std::{thread, time};
use std::time::Duration;
use std::sync::mpsc;
use std::io::Write;
use std::io::stdout;

static TEN_MILLIS: Duration = time::Duration::from_millis(500);

fn main() {
    //let (tx, rx) = mpsc::channel();
    for i in 0..10 {
     //   let tx = tx.clone();
        thread::spawn(move || {
            print!("か");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("え");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("る");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("の");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("う");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("た");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            println!("が");
            thread::sleep(TEN_MILLIS);
            print!("き");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("こ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("え");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("て");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("く");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("る");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            println!("よ");
            thread::sleep(TEN_MILLIS);

            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("わ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("わ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("わ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            println!("わ");
            thread::sleep(TEN_MILLIS);

            print!("げ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ろ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("げ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ろ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("げ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ろ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("げ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ろ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("わ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("わ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("ぐ");
            stdout().flush();
            thread::sleep(TEN_MILLIS);
            print!("わ");
            stdout().flush();
        });
        thread::sleep_ms(3500);
    }


}


