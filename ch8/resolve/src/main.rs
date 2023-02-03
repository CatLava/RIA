use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{App, Arg};
use rand;
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

fn main() {
    let app = App::new("resolve")
        .about("A simple DNS resolving tool")
        .arg(Arg::with_name("dns-server").short("s").default_value("1.1.1.1"))
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();

    let domain_name_raw = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name_raw).unwrap();

    let dns_server_raw = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse()
        .expect("invalid address");
    
    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];
}
