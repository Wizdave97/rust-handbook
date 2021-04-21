use std::{sync, thread, time};


fn main() {
    // let (tx, rx): (sync::mpsc::Sender<String>, sync::mpsc::Receiver<String>) =
    //     sync::mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         "Hello".to_string(),
    //         "World,".to_string(),
    //         "I".to_string(),
    //         "am".to_string(),
    //         "Anya Taylor Josephine Marie Joy".to_string(),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(time::Duration::new(1, 0));
    //     }
    // });

    // for received in rx {
    //     println!("{}", received);
    // }

    let counter = sync::Arc::new(sync::Mutex::new(5));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 1..10 {
        let counter = sync::Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            *counter.lock().unwrap() += 1;
            println!("Incrementing by one");
        });
        handles.push(handle);

    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
     
}
