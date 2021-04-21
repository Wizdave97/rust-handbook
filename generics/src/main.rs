mod test;
fn main() {
    let list: Vec<i32>= vec![2, 3, 4 ,5];
    println!("{}", largest(&list));
}


fn largest<'a, T: PartialOrd + std::fmt::Debug> (list: &'a Vec<T>) -> &'a T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    &largest
}