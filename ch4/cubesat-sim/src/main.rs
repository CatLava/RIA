#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

type Message = String;

struct Mailbox {
    messages: Vec<Message>
}

struct GroundStation;

#[derive(Debug)]
enum StatusMessage {
    Ok,
    NotOk,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}
fn main() {
    let sat_a = CubeSat { id:0, mailbox: Mailbox { messages: vec![] }  };
    let sat_b = CubeSat { id:1, mailbox: Mailbox { messages: vec![] }  };
    let sat_c = CubeSat { id:2, mailbox: Mailbox { messages: vec![] }  };

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);





}
