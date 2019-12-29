use clap::{Arg, App};
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::{Sender, channel};

const NAME: &str = "TCP Port Scanner";
const VERSION: &str = "v1.0";

fn main() {
    /////////////////////////////////////////////////////////////
    // Enable colored output on windows
    /////////////////////////////////////////////////////////////
    #[cfg(target_os = "windows")]
    let _enabled = ansi_term::enable_ansi_support();

    /////////////////////////////////////////////////////////////
    // Print banner & parse arguments
    /////////////////////////////////////////////////////////////
    print_banner();
    let matches = App::new(NAME)
        .version(VERSION)
        .about("A simple TCP Port Scanner")
        .author("Travis Phillips")
    .arg(Arg::with_name("port")
        .long("port")
        .short("p")
        .takes_value(true)
        .value_name("port")
        .default_value("1-1024")
        .help("Ports to scan."))
    .arg(Arg::with_name("host")
        .required(true)
        .help("Host to Scan."))
    .get_matches();

    /////////////////////////////////////////////////////////////
    // Extract some values from the matches to make life easier.
    /////////////////////////////////////////////////////////////
    let port_str = matches.value_of("port").unwrap();
    let addr_str = matches.value_of("host").unwrap().to_string();

    /////////////////////////////////////////////////////////////
    // Get a vector of the required ports to scan.
    /////////////////////////////////////////////////////////////
    let ports: Vec<u16>;
    if port_str == "-" {
        ports = (1..65535).collect();
    } else {
        ports = parse_ports(&port_str);
    }

    /////////////////////////////////////////////////////////////
    // Create the scanner threads.
    /////////////////////////////////////////////////////////////
    let (tx, rx) = channel();
    for port in ports {
        let tx = tx.clone();
        let addr_str = addr_str.clone();
        thread::spawn(move || {
            check_port(tx, &addr_str, port)
        });
    }
    drop(tx);

    /////////////////////////////////////////////////////////////
    // Listen to the threads and wait for answers on which ports
    // are open.
    /////////////////////////////////////////////////////////////
    let mut open_ports = vec![];
    for port in rx {
        open_ports.push(port);
    }

    /////////////////////////////////////////////////////////////
    // Sort the open ports and dump a report to the user.
    /////////////////////////////////////////////////////////////
    println!("\x1b[35;1m--| Open Ports on \x1b[32;1m{}\x1b[35;1m |--\x1b[0m\n", addr_str);
    open_ports.sort();
    for port in open_ports {
        println!(" [\x1b[32;1m+\x1b[0m] Port \x1b[32;1m{}\x1b[0m Open!", port);
    }
    println!("");
}

/////////////////////////////////////////////////////////////
// Print a banner.
/////////////////////////////////////////////////////////////
fn print_banner() {
    println!("\n\t\x1b[33;1m---===[ {} {} ]===---\x1b[0m\n", NAME, VERSION);
}

/////////////////////////////////////////////////////////////
// Check if we can connect to the port successfully. If so,
// send the port number over the thread's tx channel.
/////////////////////////////////////////////////////////////
fn check_port(tx: Sender<u16>, addr: &String, port: u16) {
    if let Ok(_stream) = TcpStream::connect(format!("{}:{}",addr, port)) {
        tx.send(port).unwrap();
    }
}

/////////////////////////////////////////////////////////////
// Parse the ports to be scanned that the user provided as
// a string.
/////////////////////////////////////////////////////////////
fn parse_ports(port: &str) -> Vec<u16> {
    let mut ports: Vec<u16> = Vec::new();

    /////////////////////////////////////////////////////////////
    // Split on any commas that are present.
    /////////////////////////////////////////////////////////////
    let comma_split = port.split(",");

    /////////////////////////////////////////////////////////////
    // process each comma group.
    /////////////////////////////////////////////////////////////
    for x in comma_split {
        if x.contains("-") {
            /////////////////////////////////////////////////////////////
            // If it contains a dash, atttempt to parse it as a port
            // range.
            /////////////////////////////////////////////////////////////
            let dash_split: Vec<&str> = x.split("-").collect();

            /////////////////////////////////////////////////////////////
            // Check that splitting on the dash only returns two groups
            // or panic.
            /////////////////////////////////////////////////////////////
            if dash_split.len() != 2{
                panic!("Unable to parse port information {}. Consider only one dash per range.", x);
            }

            /////////////////////////////////////////////////////////////
            // Check that low value is lower than the high value or panic.
            /////////////////////////////////////////////////////////////
            let low = dash_split[0].parse::<u16>().unwrap();
            let high = dash_split[1].parse::<u16>().unwrap();
            if high < low {panic!("Unable to parse port information {}. Low is bigger than high.", x)};

            /////////////////////////////////////////////////////////////
            // Check that low value is lower than the high value or panic.
            /////////////////////////////////////////////////////////////
            for d in low..(high+1) {
                if !ports.contains(&d) {
                    ports.push(d);
                }
            }
        } else {
            /////////////////////////////////////////////////////////////
            // If it doesn't contains a dash, atttempt to parse it as a
            // stand alone port number.
            /////////////////////////////////////////////////////////////
            let n = x.parse::<u16>().unwrap();
            if !ports.contains(&n) {
                ports.push(n);
            }
        }
    }
    /////////////////////////////////////////////////////////////
    // Sort the ports and return them.
    /////////////////////////////////////////////////////////////
    ports.sort();
    return ports;
}
