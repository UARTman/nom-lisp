use runtime::Runtime;
use std::io::{stdin, BufRead};

mod ast;
mod parser;
mod runtime;

fn main() {
    let mut runtime = Runtime::try_new().unwrap();
    'mainloop: loop {
        let mut buf = String::new();
        let node = loop {
            let line = stdin().lock().lines().next().unwrap().unwrap();
            buf.push_str(&line);
            buf.push('\n');
            let parsed = parser::node(buf.as_bytes());
            match parsed {
                Ok((_, node)) => break node,
                Err(e) => match e {
                    nom::Err::Incomplete(_) => continue,
                    _ => {
                        println!("{:?}", e);
                        continue 'mainloop;
                    }
                },
            }
        };

        let result = runtime.eval(node);
        match result {
            Ok(r) => println!("{r:?}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
