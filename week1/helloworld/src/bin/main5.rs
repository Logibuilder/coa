use std::str;
use std::fs::File;
use std::io::Read;
use std::net::IpAddr;

fn main() {


   let ips = get_addrs("ips.txt");
    match ips {
         Ok(ips) => println!("IPs: {:?}", ips),
         Err(e) => println!("Error: {:?}", e)
    }
}


#[derive(Debug)]
enum Error {
    Io(std::io::Error),
}

/// Reads a file that contains an address per line and returns a Vector with all well-formed
/// addresses and prints a warning on the standard error output for every malformed lines.
///
/// Addresses should be ipv4 or ipv6 addresses. 
fn get_addrs(path: &str) -> Result<Vec<IpAddr>, Error> { 

    let mut file = File::open(path).map_err(Error::Io)?;
    let mut content = String::new();

    file.read_to_string(&mut content).map_err(Error::Io)?;

    println!("Content of the file: {}", content);
    let mut ips = Vec::new();
    for line in content.lines() {
        match line.parse::<IpAddr>() {
            Ok(ip) => ips.push(ip),
            Err(_) => eprintln!("Warning: Malformed IP address: {:?}", line)
        }
    }
    Ok(ips)
 }