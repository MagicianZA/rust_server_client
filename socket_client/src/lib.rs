#[cfg(test)]
mod tests {
    use crate::socket_client::start_client;

    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_client() {
        socket_client::start_client();
        assert_eq!(1, 1);
    }
}


pub mod socket_client
{
    pub fn start_client()
    {
        match std::net::TcpStream::connect("127.0.0.1:7878"){
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}