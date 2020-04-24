mod common;
mod graphics;
mod ui;

use common::rect::Rect;
use graphics::window::Window;

fn main() {
    let rect = Rect::new((0, 0), (20, 40));
    println!("{:?}", rect);

    assert!(rect.intersects_point((0, 0)));
    assert!(rect.intersects_point((10, 30)));
    assert!(!rect.intersects_point((20, 15)));

    let window = Window::new("App", (300, 400));
    window.run(move |render| {
        render.draw(&rect);
    });
}
