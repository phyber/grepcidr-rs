extern crate clap;
extern crate ipnet;

use std::{
    fs::File,
    io,
    io::BufReader,
    io::prelude::*,
    net::IpAddr,
    process::exit,
};
use ipnet::{
    Contains,
    IpNet,
};

fn grep<T: BufRead + Sized>(reader: T, cidr: &IpNet) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        match to_ipaddr(&line) {
            Some(ip) => {
                if cidr.contains(&ip) {
                    println!("{}", line);
                }
            },
            None => continue,
        }
    }
}

fn to_ipnet(net_str: &str) -> Option<IpNet> {
    match net_str.parse() {
        Ok(net) => Some(net),
        Err(_) => None,
    }
}

fn to_ipaddr(ip_str: &str) -> Option<IpAddr> {
    match ip_str.parse() {
        Ok(ip) => Some(ip),
        Err(_) => None,
    }
}

fn main() {
    let args = clap::App::new("grepcidr")
        .version("0.1")
        .about("grep for network CIDRs")
        .arg(clap::Arg::with_name("cidr")
             .help("CIDR to grep for")
             .takes_value(true)
             .required(true))
        .arg(clap::Arg::with_name("input")
             .help("File to search, STDIN by default")
             .takes_value(true)
             .default_value("-")
             .required(false))
        .get_matches();

    // Required arg, should be safe to unwrap.
    let cidr = match to_ipnet(&args.value_of("cidr").unwrap()) {
        Some(cidr) => cidr,
        None => {
            eprintln!("Malformed CIDR");
            exit(1);
        },
    };

    // Arg has a default, should be safe to unwrap.
    let input = args.value_of("input").unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        grep(reader, &cidr);
    }
    else {
        // TODO: Handle open error
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        grep(reader, &cidr);
    }
}
