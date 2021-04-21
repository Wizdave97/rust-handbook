#[macro_export]
macro_rules! my_vec  {
    ( $( $x:expr ), *) => {
       {
            let mut temp = Vec::new();
        $(
          temp.push($x);
        )*
        temp
    }
    };
}

pub trait HelloMacro {
    fn hello_macro() -> ();
}
