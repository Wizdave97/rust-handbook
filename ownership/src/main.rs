#[derive(Debug)]
struct Rect {
  width: f64,
  height: f64,
}
impl Rect {
  fn area (&self) -> f64 {
    self.width * self.height
  }
}
#[derive(Debug)]
enum UsState {
  Alaska,
  Alabama,
  California,
  Seattle
}
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}
fn main() {
  let rectangle = Rect {
    width: 20.0,
    height: 30.0
  };
  println!("Rect is {:?} with area {}", rectangle, rectangle.area());
  println!("Coin Value: {}", coin_value(Coin::Quarter(UsState::California)));
  
}

fn coin_value(coin: Coin) -> u32 {
  match coin {
    Coin::Nickel => 1,
    Coin::Dime => 0,
    Coin::Penny => 2,
    Coin::Quarter(i) => {
      println!("Coin Quarter from {:?}", i);
      25
    }
  }
}
