mod test;
struct MyBox<T>(T);

impl<T> MyBox<T>  {
    fn new(i: T) -> MyBox<T> {
        MyBox(i)
    }
}
impl<T> Drop for MyBox<T>  {
    fn drop(&mut self) {
       println!("Dropping Box")
    }
}

fn main() {
    let my_box = MyBox::new(4);

    println!("{}", my_box.0);
}
