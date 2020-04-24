mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;

fn main() {
    let rect = Rect::new((20, 40), (80, 40));

    Window::new("App", (600, 400)).run(move |render| {
        render.draw(&rect);
    });
}
