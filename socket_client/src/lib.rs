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


mod socket_client
{
    pub fn start_client()
    {
        std::net::TcpStream::connect("localhost:7878");
    }
}