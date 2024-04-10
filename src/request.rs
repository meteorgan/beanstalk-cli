use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use bufstream::BufStream;

pub struct Request {
    stream: BufStream<TcpStream>
}

impl Request {
    pub fn new(ip: &str, port: i32) -> Result<Request, String> {
        let addr = format!("{}:{}", ip, port);
        let r = TcpStream::connect(addr);
        match r {
            Ok(stream) => {
                let buf_stream = BufStream::new(stream);
                Ok(Request{stream: buf_stream})
            }
            Err(err) => Err(err.to_string())
        }
    }

    pub fn write_data(&mut self, data: &[u8]) {
        self.stream.write_all(data).unwrap();
        self.stream.write_all("\r\n".as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn read_line(&mut self) -> String {
        let mut buf = String::new();
        self.stream.read_line(&mut buf).unwrap();

        String::from(buf.trim())
    }

    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let mut buffer = vec![0; size];
        self.stream.read_exact(&mut buffer).unwrap();
        buffer
    }
}