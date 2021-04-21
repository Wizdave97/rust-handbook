
#[cfg(test)]
mod tests {
     use std::rc::Rc;
     use std::cell::RefCell;

    use smart_pointers::limit_tracker;
    
     

     struct  MockMessenger<'a> {
         messages: Rc<RefCell<Vec<&'a str>>>
     }

     impl<'a> MockMessenger<'a> {
         fn new() -> MockMessenger<'a> {
            MockMessenger {
                messages: Rc::new(RefCell::new(vec![]))
            }
         }
     }

     impl<'a> limit_tracker::Messenger<'a> for MockMessenger<'a> {
         fn send(&self, msg: &'a str) {
            self.messages.borrow_mut().push(msg.trim());
         }
     }

     #[test]
     fn it_should_send_message_when_user_has_reached_75_of_quota() {
         let messenger = MockMessenger::new();

         let mut quota_tracker = limit_tracker::LimitTracker::new(100, 50, &messenger);

         quota_tracker.set_value(75);
         
         assert_eq!(messenger.messages.borrow()[0], "You have used up 75% of your available quota");

     }
}