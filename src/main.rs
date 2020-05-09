mod common;
mod graphics;
mod ui;

use common::Rect;
use graphics::Window;
use ui::*;

fn main() {
    let window = Window::new("App", (600, 400), 2);

    let rect = Col::red(Rect::new((31, 31), (2, 2)));
    let dot = Rect::new((30, 30), (1, 1));
    let img = Pos::right_bot(4, 4, Image::new("./data/1.png", window.render()));
    let u = Pos::left(4, Image::new("./data/u.png", window.render()));
    let text = Col::green("jie adding the КАПС README.,!:;|*@ Привет, мир ЪЁь!1230");
    let hello = Col::blue("Hello, world! MOW");
    let cyr = Pos::right(0, Col::red("`ж`эьмъйы ЭЬМЪЙЫ"));

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&dot);
        render.draw(&img);
        render.draw(&Pos::left_top(0, 0, &text));
        render.draw(&hello);
        render.draw(&cyr);
        render.draw(&u);
    });
}
