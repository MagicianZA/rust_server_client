use core::time;
use std::thread;

fn main() {
    //use socket_client;
    println!("Hello, world!");

    let server_thread = thread::spawn(move|| {
        // connection succeeded
        socket_server::socket_server::start_server()
    });

    server_thread.join().unwrap();

    let ten_millis = time::Duration::from_millis(2000);
    thread::sleep(ten_millis);
    
    
    socket_client::socket_client::start_client();

    thread::sleep(ten_millis);

}
