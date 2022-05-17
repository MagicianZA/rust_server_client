#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_socket()
    {
        println!("hello");
        socket_server::start_server();
        assert_eq!(1, 1);
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