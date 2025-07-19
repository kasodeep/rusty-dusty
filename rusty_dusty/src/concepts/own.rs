use std::rc::Rc;

/// Represents a CubeSat with a unique identifier.
#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

/// A mailbox that stores messages for CubeSats.
#[derive(Debug, Clone)]
struct Mailbox {
    messages: Vec<Message>,
}

/// A message addressed to a specific CubeSat.
#[derive(Debug, Clone)]
struct Message {
    to: u64,
    content: String,
}

/// GroundStation is responsible for creating CubeSats and sending messages.
struct GroundStation;

impl Mailbox {
    /// Adds a message to the mailbox.
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    /// Delivers a message addressed to the given CubeSat, if available.
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in (0..self.messages.len()).rev() {
            if self.messages[i].to == recipient.id {
                return Some(self.messages.remove(i));
            }
        }
        None
    }
}

impl GroundStation {
    /// Connects to a CubeSat with the given ID.
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    /// Sends a message using a mutable reference to the mailbox.
    ///
    /// This avoids taking ownership of the mailbox and allows reuse.
    fn send_with_reference(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    /// Sends a message by cloning the mailbox and returning the modified copy.
    ///
    /// Useful when ownership cannot be transferred but modifications are needed.
    fn send_with_clone(&self, mailbox: Mailbox, msg: Message) -> Mailbox {
        let mut mailbox_clone = mailbox.clone();
        mailbox_clone.post(msg);
        mailbox_clone
    }

    /// Sends a message by taking and returning ownership of the mailbox.
    ///
    /// This allows temporary ownership without cloning.
    fn send_with_refactor(&self, mailbox: Mailbox, msg: Message) -> Mailbox {
        let mut mailbox = mailbox;
        mailbox.post(msg);
        mailbox
    }

    /// Sends a message using an `Rc` for shared ownership.
    ///
    /// Since `Mailbox::post` requires a mutable reference, we clone the `Rc`
    /// and operate on the underlying data. This method assumes the mailbox is immutable
    /// (not shared mutable). Use `Rc<RefCell<Mailbox>>` for shared mutability.
    fn send_with_rc(&self, mailbox: Rc<Mailbox>, msg: Message) {
        let mut mailbox_clone = mailbox.as_ref().clone();
        mailbox_clone.post(msg);
    }
}

impl CubeSat {
    /// Receives a message addressed to this CubeSat from the mailbox.
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }

    /// Logs the number of messages in the mailbox for this CubeSat.
    fn log_status(&self, mailbox: &Mailbox) {
        let count = mailbox
            .messages
            .iter()
            .filter(|msg| msg.to == self.id)
            .count();
        println!("CubeSat {} has {} pending messages.", self.id, count);
    }
}

/// Fetches a list of CubeSat IDs to be used in the simulation.
fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

pub fn main() {
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();

    // Create an empty Mailbox
    let mut mailbox = Mailbox {
        messages: vec![],
    };

    println!("=== Method 1: Using References ===");
    for sat_id in sat_ids.iter() {
        let sat = base.connect(*sat_id);
        let msg = Message {
            to: *sat_id,
            content: format!("Hello, CubeSat {}!", sat_id),
        };
        base.send_with_reference(&mut mailbox, msg);
        sat.log_status(&mailbox);
    }
    println!("Mailbox after reference method: {:?}", mailbox);

    println!("\n=== Method 2: Cloning ===");
    for sat_id in sat_ids.iter() {
        let sat = base.connect(*sat_id);
        let msg = Message {
            to: *sat_id,
            content: format!("Cloned msg for CubeSat {}!", sat_id),
        };
        mailbox = base.send_with_clone(mailbox, msg);
        sat.log_status(&mailbox);
    }
    println!("Mailbox after clone method: {:?}", mailbox);

    println!("\n=== Method 3: Refactoring to Return Ownership ===");
    for sat_id in sat_ids.iter() {
        let sat = base.connect(*sat_id);
        let msg = Message {
            to: *sat_id,
            content: format!("Refactored msg for CubeSat {}!", sat_id),
        };
        mailbox = base.send_with_refactor(mailbox, msg);
        sat.log_status(&mailbox);
    }
    println!("Mailbox after refactor method: {:?}", mailbox);

    println!("\n=== Method 4: Using Rc for Shared Ownership ===");
    let mailbox_rc = Rc::new(mailbox);
    for sat_id in sat_ids.iter() {
        let sat = base.connect(*sat_id);
        let msg = Message {
            to: *sat_id,
            content: format!("Rc msg for CubeSat {}!", sat_id),
        };
        base.send_with_rc(Rc::clone(&mailbox_rc), msg);
        sat.log_status(mailbox_rc.as_ref());
    }
    println!("Mailbox after Rc method: {:?}", mailbox_rc);

    println!("\n=== Receiving Messages ===");
    // Clone Rc inner for mutable access to receive messages
    let mut mailbox = mailbox_rc.as_ref().clone();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        if let Some(msg) = sat.recv(&mut mailbox) {
            println!("CubeSat {} received: {:?}", sat_id, msg);
        } else {
            println!("CubeSat {}: No messages.", sat_id);
        }
    }
}
