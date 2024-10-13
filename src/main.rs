use std::str::FromStr;
use std::{io, thread};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;


fn main(){
    // 127.0.0.1 8000-8080
    println!("Enter IP adress and Port Range ");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    let [ip, port_range]: [String; 2] = inp.split(" ").into_iter().map(|x| x.to_string()).collect::<Vec<String>>().try_into().unwrap();
    let [ranges, rangef]: [u16; 2] = port_range.split("-").into_iter().map(|x| x.trim().parse().unwrap()).collect::<Vec<u16>>().try_into().unwrap();
    
    let thread_one_ip = ip.clone();
    let scan = thread::spawn( move || for port in ranges..=rangef{
            let addr = SocketAddr::from_str(format!("{}:{:?}", thread_one_ip, port ).as_str()).unwrap();
            match TcpStream::connect_timeout(&addr,Duration::from_millis(3)){
                Ok(_) => println!("{} Okay ", addr ),
                Err(_) => println!("{} Not Okay ", addr),
                
            }
    });
    
    scan.join().unwrap();

}