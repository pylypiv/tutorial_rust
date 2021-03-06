use std::io;
use std::{env, process, thread};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::io::Write;


const MAX: u16 = 65535;

struct Arguments {
    flags:   String,
    ipaddr:  IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enought argyment");
        }else if args.len() > 4 {
            return Err("too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipadr) = IpAddr::from_str(&f){
            return Ok(Arguments{ flags: String::from(""), ipaddr: ipadr, threads: 4});
        }else {

            let flags = args[1].clone();
            if flags.contains("-h") || flags.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want \r\n -h or --help to show this message ");
                return Err("help");
            }else if flags.contains("-h") || flags.contains("--help") {
                return Err("contains too many arguments");
            }else if flags.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                     Ok(s) => s,
                     Err(_) => return Err("not valid ip addres")
                 };
                 let threads = match args[2].parse::<u16>(){
                    Ok(s)   => s,
                    Err(_)  => return Err("failed to parse")
                };
                return Ok(Arguments{threads, flags, ipaddr});
            }else{
                return Err("Invalid Syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, ipadr: IpAddr, num_threads: u16){
        let mut port =  start_port + 1;
        loop {
            match TcpStream::connect((ipadr, port)) {
                Ok(_) => {
                    print!(".");
                    io::stdout().flush().unwrap();
                    tx.send(port).unwrap();
                }
                Err(_) => {}
            }

            if (MAX - port) <= num_threads {
                break;
            }

            port = port + num_threads;
        }
}

fn main() {
     let cl : Vec<String> =  env::args().collect();
     let program = cl[0].clone();
     for str in &cl{
        println!("{}",&str);
     }

     let arguments = Arguments::new(&cl).unwrap_or_else( |err|{
            if err.contains("help"){
                process::exit(0);
            }else{
                eprintln!("{} problem parsing argument {}", program, err);
                process::exit(0);
            }
     });

    let  num_threads = arguments.threads;
    let  ip_adr = arguments.ipaddr;
    let (sender , reciever) =  channel();
    for i in 0..num_threads {
        let tx = sender.clone();

        thread::spawn(move ||{
            scan(tx, i, ip_adr, num_threads);
        });
    }

    let mut out = vec![];
    drop(sender);
    for p in reciever {
        out.push(p);
    }

    println!();
    out.sort();
    for v in out {
        println!("{} is open", v);
    }

}
   