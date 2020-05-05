mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;
use ui::{Image, Color};

fn main() {
    let window = Window::new("App", (600, 400));

    let rect = Color::red(Rect::new((30, 30), (80, 40)));
    let img = Image::new("data/1.png", window.render());
    let hello = Color::green("by adding the КАПС README.!@#$%^&* Привет, мир ЪЁь!1230");

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&img);
        render.draw(&hello);
    });
}
