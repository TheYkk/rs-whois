use std::io::{
    Read,
    Write,
};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() -> Result<(), std::io::Error> {
    // Open new tcp connection to whois server
    let mut con = TcpStream::connect("whois.nic.uno:43")?;

    // Send domain name with new line
    con.write_all("kaan.uno \r\n".as_bytes())?;

    // Set buffer to 10Kb
    let mut dat = vec![0u8; 1024 * 10];

    // IMPORTANT read till end, because some times whois server
    // sends multiple whois data that splited.
    let c = con.read_to_end(&mut dat).unwrap();

    // Print whois data
    println!("{}", from_utf8(&dat).unwrap());
    println!("Total bytes: {}", c);
    Ok(())
}
