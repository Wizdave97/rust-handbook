mod help;
use help::helpers;

fn main() {
    let mut list = vec![34, 21, 49, 5, 33, 10];
    let mmm = helpers::mean_mode_median(&mut list);
    println!("{:?}", mmm);

    println!("{}", helpers::to_pig_latin("The quick (\"brown\") fox can't jump 32.3 feet, right?"));
    helpers::dept()
}






