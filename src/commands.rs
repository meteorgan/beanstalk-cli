use std::str::from_utf8;
use crate::request;
use crate::cli;

pub struct Commands {
    transport: request::Request
}

impl Commands {
    pub fn new(req: request::Request) -> Commands {
        Commands{transport: req}
    }

    pub fn execute(&mut self, input: &str) -> bool {
        let command = Commands::get_command(input);

        match command {
            "quit" => {
                return false;
            },
            "stats" => self.stats(input),
            "put" => self.put_job(input),
            "stats-job" => self.stats_job(input),
            "delete" => self.delete(input),
            "reserve" => self.reserve(input),
            "reserve-with-timeout" => self.reserve_with_timeout(input),
            "reserve-job" => self.reserve_job(input),
            "peek" => self.peek(input),
            "kick" => self.kick(input),
            "kick-job" => self.kick_job(input),
            "list-tubes" => self.list_tubes(input),
            "bury" => self.bury(input),
            _ => println!("unknown command: {}", input)
        }
        return true;
    }

    fn get_command(line: &str) -> &str {
        return line.split_whitespace().nth(0).unwrap();
    }

    fn stats(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());

        let buf = self.transport.read_line();
        if !buf.starts_with("OK") {
            eprintln!("err: {}", buf);
            return;
        }

        if let Some(bytes) = buf.split_whitespace().nth(1) {
            let size = bytes.parse::<usize>().unwrap();
            let buffer = self.transport.read_bytes(size+2);
            println!("{}", from_utf8(buffer.as_slice()).unwrap().trim());
        } else {
            eprintln!("err: {}", buf);
        }
    }

    fn put_job(&mut self, input: &str) {
        if let Some(bytes) = input.split_whitespace().nth(4) {
            let size = bytes.parse::<usize>().unwrap();
            let data = cli::read_bytes_from_cli(size);
            cli::read_line_from_cli();
            self.transport.write_data(input.as_bytes());
            self.transport.write_data(&data);

            let response = self.transport.read_line();
            println!("{}", response);
        } else {
            eprintln!("must in format: put <pri> <delay> <ttr> <bytes>");
        }
    }

    fn stats_job(&mut self, input: &str) {
        if input.split_whitespace().nth(1).is_some() {
            self.transport.write_data(input.as_bytes());

            let buf = self.transport.read_line();
            if buf.starts_with("NOT_FOUND") {
                println!("{}", buf.trim());
            } else {
                let size = buf.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
                let buffer = self.transport.read_bytes(size+2);
                println!("{}", from_utf8(buffer.as_slice()).unwrap().trim());
            }
        } else {
            eprintln!("must in format: stats-job <id>");
        }
    }

    fn delete(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());
        let response = self.transport.read_line();
        println!("{}", response);
    }

    fn reserve(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());

        let first_response = self.transport.read_line();
        if !first_response.starts_with("RESERVED") {
            println!("{}", first_response);
            return;
        }

        let size = first_response.split_whitespace().nth(2).unwrap().parse::<usize>().unwrap();
        let bytes = self.transport.read_bytes(size+2);
        println!("{}", first_response);
        println!("{}", from_utf8(bytes.as_slice()).unwrap().trim());
    }

    fn reserve_with_timeout(&mut self, input: &str) {
        let timeout = input.split_whitespace().nth(1);
        match timeout {
            Some(timeout_str) => {
                if timeout_str.parse::<usize>().is_err() {
                    eprintln!("timeout must be usize");
                }
                self.reserve(input);
            }
            None => eprintln!("must be in format: reserve-with-timeout <seconds>")
        }
    }

    fn reserve_job(&mut self, input: &str) {
        if let Some(id) = input.split_whitespace().nth(1) {
            if id.parse::<usize>().is_err() {
                eprintln!("job id must be usize");
            }
            self.reserve(input);
        } else {
            eprintln!("must be in format: reserve-job <id>")
        }
    }

    fn peek(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());
        let response = self.transport.read_line();
        if !response.starts_with("FOUND") {
            println!("{}", response);
            return;
        }

        let size = response.split_whitespace().nth(2).unwrap().parse::<usize>().unwrap();
        let bytes = self.transport.read_bytes(size+2);
        println!("{}", response);
        println!("{}", from_utf8(bytes.as_slice()).unwrap().trim());
    }

    fn kick(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());

        let response = self.transport.read_line();
        println!("{}", response);
    }

    fn kick_job(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());
        let response = self.transport.read_line();
        println!("{}", response);
    }

    fn list_tubes(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());

        let response = self.transport.read_line();
        let size = response.split_whitespace().nth(1).unwrap().parse::<usize>().unwrap();
        let bytes = self.transport.read_bytes(size+2);
        println!("{}", response);
        println!("{}", from_utf8(bytes.as_slice()).unwrap().trim());
    }

    fn bury(&mut self, input: &str) {
        self.transport.write_data(input.as_bytes());

        let response = self.transport.read_line();
        println!("{}", response);
    }
}