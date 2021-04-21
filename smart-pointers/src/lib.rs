
pub mod limit_tracker {
    pub trait Messenger<'a> {
        fn send(&self, msg: &'a str);
    }

    pub struct LimitTracker<'a, T: Messenger<'a>> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T: Messenger<'a>> LimitTracker<'a, T> {
        pub fn new(max: usize, value: usize, messenger: &'a T) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value,
                max,
            }
        }

        pub fn set_value(&mut self, val: usize) {
            self.value = val;

            let percentage = self.value as f64 / self.max as f64;

            if percentage >= 1.0 {
                self.messenger.send(
                    "
                You have exceeded your available quota
                ",
                );
            } else if percentage >= 0.9 {
                self.messenger.send(
                    "
                You have used up 90% of your quota",
                );
            } else if percentage >= 0.75 {
                self.messenger.send(
                    "
                You have used up 75% of your available quota
                ",
                );
            }
        }
    }
}
