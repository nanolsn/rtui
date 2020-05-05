mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;
use ui::image::Image;

fn main() {
    let window = Window::new("App", (600, 400));

    let rect = Rect::new((30, 30), (80, 40));
    let img = Image::new("data/1.png", window.render());
    let hello = String::from("by adding the fucking README.!@#$%^&* Привет, мир ЪЁь!1230");

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&img);
        render.draw(&hello);
    });
}
