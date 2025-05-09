// r22_refcell
// Learning rust 2024, Smart Pointers 3: Interior mutability pattern
//
// 2025-01-10   PV

/*
Version that works with Messenger send using a mutable ref

pub trait Messenger {
    fn send(&mut self, msg: &str);
}


struct LimitTracker<'a, T: Messenger> {
    messenger: &'a mut T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T:Messenger {
    pub fn new(messenger: &'a mut T, max: usize) -> Self {
        Self {messenger, max, value:0}
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        self.messenger.send(format!("set_value: {}", value).as_str());
    }
}

struct M {}

impl M {
    fn new() -> Self { M{} }
}

impl Messenger for M {
    fn send(&mut self, msg: &str) {
        println!("Message: {msg}");
    }
}

fn main() {
    let mut m = M::new();
    let mut l = LimitTracker::new(&mut m, 12);
    l.set_value(80);
}
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // Using RefCell<T> to mutate an inner value while the outer value is considered immutable
    struct MockMessenger {
        //sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                //sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //self.sent_messages.push(String::from(message));     // Not allowed if sent_messages is a Vec<String>

            // RefCell allow many immutable borrows or a single mutable borrow an any point in time
            // This if checked at run-tipe, not as compile time
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
