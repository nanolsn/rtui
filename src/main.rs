mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;
use ui::{Image, Col, Pos};

fn main() {
    let window = Window::new("App", (600, 400));

    let rect = Col::red(Rect::new((30, 30), (80, 40)));
    let img = Pos::right_bot(30, 10, Image::new("data/1.png", window.render()));
    let text = Col::green("jie adding the КАПС README.,!:;|*@ Привет, мир ЪЁь!1230");
    let hello = Col::blue("Hello, world!");
    let cyr = Pos::right(0, Col::red("жэьмъйы"));

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&img);
        render.draw(&Pos::left_top(0, 0, &text));
        render.draw(&hello);
        render.draw(&cyr);
    });
}
