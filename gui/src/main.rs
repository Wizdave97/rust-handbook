use gui::{self, gui_lib};
fn main() {
    let screen = gui_lib::Screen::new(vec![
        Box::new(gui::Selectbox::new(
            "Races",
            100.0,
            50.0,
            vec!["single", "co-op", "online"]
            
        )),
        Box::new(gui::Button::new("Start", 60.0, 35.0))
    ]);

    screen.render();
}