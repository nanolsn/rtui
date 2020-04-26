mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;
use ui::image::Image;

fn main() {
    let window = Window::new("App", (600, 400));

    let rect = Rect::new((20, 40), (80, 40));
    let img = Image::new("data/1.png", window.render());

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&img);
    });
}
