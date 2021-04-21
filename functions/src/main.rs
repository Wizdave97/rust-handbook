fn main() {
    println!("Hello, world!");
    let temp = celsius_farenheit(20.0, 'F');
    let nth_fibonacci = fibonacci(7);
    println!(
        "Your temp is {} and your fibo number is {}",
        temp, nth_fibonacci
    );
    twelve_days_of_christmas();
}

fn celsius_farenheit(x: f64, to: char) -> f64 {
    if to == 'C' || to == 'c' {
        let farenheit = (1.8 * x) + 32.0;
        return farenheit;
    } else if to == 'F' || to == 'f' {
        let celsius = (x - 32.0) / 1.8;
        return celsius;
    } else {
        return 0.0;
    }
}

fn fibonacci(x: u64) -> u64 {
    let mut arr = vec![0, 1];
    if x <= 0 {
        return arr[0];
    }
    if x <= 1 {
        return arr[1];
    }
    for index in 0..(x) {
        if index < 2 {
            continue;
        }
        arr.push(arr[(index - 1) as usize] + arr[(index - 2) as usize]);
    }
    return arr[(x - 1) as usize];
}
fn twelve_days_of_christmas() {
    let arr = [
        "2 drummers drumming",
        "Eleven pipers piping",
        "Ten lords a leaping",
        "Nine ladies dancing",
        "Eight maids a milking",
        "Seven swans a swimming",
        "Six geese a laying",
        "Five gold rings, badam-pam-pam",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "A partridge in a pear tree",
    ];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut index = 0;
    for i in (0..12).rev() {
        println!("On the {} day of Christmas\nMy true love gave to me", days[index]);
        for j in i..12 {
            if index > 0 && j >= 11 {
                println!("And {}", arr[j]); 
                continue
            }
            println!("{}", arr[j]);
        }
        index += 1;
        println!("")
    }
}
