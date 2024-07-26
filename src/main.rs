use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{Arg, App};


fn process_line<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines(){
        let line = line_.unwrap();
        match re.find(&line){
            Some(_) => println!("{}", line),
            None => continue,
        }
    }
}

fn main(){
    let args = App::new("grep_clone")
        .version("0.1.0")
        .author("Rose Tovar")
        .about("A simple grep clone")
        .arg(Arg::with_name("pattern")
            .short("p")
            .long("pattern")
            .value_name("PATTERN")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("The file to search in")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let file = args.value_of("file").unwrap_or("-");

    if file == "-"{
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_line(reader, re);
    } else {
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        process_line(reader, re);
    }
}