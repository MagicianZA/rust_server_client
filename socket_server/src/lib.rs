#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod socket_server
{
    pub fn start_server()
    {
        let listener = std::net::TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established! {:?}", stream.local_addr());
            break;
        }
    } 
}