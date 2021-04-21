pub mod gui_lib {
    pub trait Draw {
        fn draw(&self) -> ();
    }

    pub struct Screen {
        components: Vec<Box<dyn Draw>>
    }

    impl Screen {
        pub fn new(components: Vec<Box<dyn Draw>>) -> Screen {
            Screen{
                components
            }
        }
        pub fn render(&self) -> () {
            for comp in self.components.iter() {
                comp.draw();
            }
        }
    }

}
#[derive(Debug)]
pub struct Selectbox<'a> {
    label: &'a str,
    width: f64,
    height: f64,
    options: Vec<&'a str>,
}

impl<'a> Selectbox<'a> {
    pub fn new(label: &'a str, width: f64, height: f64, options: Vec<&'a str>) -> Selectbox<'a> {
        Selectbox {
            label,
            width,
            height,
            options
        }
    }
}

impl<'a> gui_lib::Draw for Selectbox<'a> {
    fn draw(&self) -> () {
        println!("{:#?}", self)
    }
}
#[derive(Debug)]
pub struct Button<'a> {
    label: &'a str,
    width: f64,
    height: f64,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str, width: f64, height: f64) -> Button<'a> {
        Button{
            label,
            width,
            height
        }
    }
}

impl<'a> gui_lib::Draw for Button<'a> {
    fn draw(&self) -> () {
        println!("{:#?}", self) 
    }
}