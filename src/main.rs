use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::str::from_utf8;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:11300") {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let input = read_line_from_cli();
            let command = get_command(&input);

            match command{
                "exit" => exit(0),
                "stats" => stats(&mut stream, &input),
                "put" => put_job(&mut stream, &input),
                "stats-job" => stats_job(&mut stream, &input),
                _ => println!("unknown command: {}", input)
            }
        }
    } else {
        eprintln!("connect beanstalk error!");
        exit(-1);
    }
}

fn read_line_from_cli() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return String::from(line.trim());
}

fn read_bytes_from_cli(size: usize) -> Vec<u8> {
    let mut buffer = vec![0; size];
    io::stdin().read_exact(&mut buffer).unwrap();
    return buffer;
}

fn get_command(line: &str) -> &str {
    return line.split_whitespace().nth(0).unwrap();
}

fn write_data(stream: &mut TcpStream, data: &[u8]) {
    stream.write(data).unwrap();
    stream.write("\r\n".as_bytes()).unwrap();
}

fn read_line(stream: &mut TcpStream) -> String {
    let mut buffer_reader = BufReader::new(stream);
    let mut buf = String::new();
    buffer_reader.read_line(&mut buf).unwrap();

    return String::from(buf.trim());
}

fn read_bytes(stream: &mut TcpStream, size: usize) -> Vec<u8> {
    let mut buffer = vec![0; size];
    stream.read_exact(&mut buffer).unwrap();
    return buffer;
}

fn stats(stream: &mut TcpStream, input: &str) {
    write_data(stream, input.as_bytes());

    let mut buffer_reader = BufReader::new(stream);
    let mut buf = String::new();
    buffer_reader.read_line(&mut buf).unwrap();
    if !buf.starts_with("OK") {
        eprintln!("err: {}", buf);
        return;
    }

    if let Some(bytes) = buf.split_whitespace().nth(1) {
        let size = bytes.parse::<usize>().unwrap();
        let mut buffer = vec![0; size];
        buffer_reader.read_exact(&mut buffer).unwrap();
        println!("{}", from_utf8(buffer.as_slice()).unwrap().trim());
    } else {
        eprintln!("err: {}", buf);
    }
}

fn put_job(stream: &mut TcpStream, input: &str) {
    if let Some(bytes) = input.split_whitespace().nth(4) {
        let size = bytes.parse::<usize>().unwrap();
        let data = read_bytes_from_cli(size);
        read_line_from_cli();
        write_data(stream, input.as_bytes());
        write_data(stream, &data);

        let response = read_line(stream);
        println!("{}", response);
    } else {
        eprintln!("must in format: put <pri> <delay> <ttr> <bytes>");
    }
}

fn stats_job(stream: &mut TcpStream, input: &str) {
    if let Some(id_str) = input.split_whitespace().nth(1) {
        let id = id_str.parse::<usize>().unwrap();
        write_data(stream, input.as_bytes());

        let mut buffer_reader = BufReader::new(stream);
        let mut buf = String::new();
        buffer_reader.read_line(&mut buf);
        if buf.starts_with("NOT_FOUND") {
            println!("{}", buf.trim());
        } else {
            let size = buf.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
            let mut buffer = vec![0; size];
            buffer_reader.read_exact(&mut buffer).unwrap();
            println!("{}", from_utf8(buffer.as_slice()).unwrap().trim());
        }
    } else {
        eprintln!("must in format: stats-job <id>");
    }
}