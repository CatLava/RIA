#![allow(unused_variables)]
use std::rc::Rc;
use std::cell::RefCell;


// Derive is simply an impl on the struct
// clone and copy can be expensive on memory
// clone more so
#[derive(Debug, Clone)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message>{
        self.mailbox.messages.pop()
    }
}

type Message = String;

#[derive(Debug, Clone)]
struct Mailbox {
    messages: Vec<Message>
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64 // Mhz
}

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message,){
        to.mailbox.messages.push(msg);
    }
}

#[derive(Debug, Copy, Clone)]
enum StatusMessage {
    Ok,
    NotOk,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}
fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
        GroundStation{radio_freq: 87.65}));
    let mut sat_a = CubeSat { id:0, mailbox: Mailbox { messages: vec![] }  };
    let sat_b = CubeSat { id:1, mailbox: Mailbox { messages: vec![] }  };
    let sat_c = CubeSat { id:2, mailbox: Mailbox { messages: vec![] }  };

    println!("{:?}", base);
    //let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    {
    let mut base_2 = base.borrow_mut();
    base_2.radio_freq -= 12.34;
    println!("base_2: {:?}", base_2);
    }

    //base.send(&mut sat_a, "Hello satellite".to_string());

    //println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
    println!("t1: {:?}", sat_a);
    let msg = sat_a.recv();
    println!("msg: {:?}", msg);

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 46.78;
    println!("base3: {:?}", base_3);
    println!("base: {:?}", base);



    // let a_status = check_status(sat_a);
    // let b_status = check_status(sat_b);
    // let c_status = check_status(sat_c);

   // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);





}
