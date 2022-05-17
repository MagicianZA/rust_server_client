use std::{thread, time};

fn main() {
    //use socket_client;
    println!("Hello, world!");

    thread::Builder::new().name("thread1".to_string()).spawn(move|| {
        // connection succeeded
        socket_server::socket_server::start_server()
    });

    let ten_millis = time::Duration::from_millis(2000);
    thread::sleep(ten_millis);
    
    
    socket_client::socket_client::start_client();

    thread::sleep(ten_millis);

}
