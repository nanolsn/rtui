mod common;
mod graphics;
mod ui;

use common::{Rect, Color};
use graphics::{Window, FontStyle};
use ui::*;

fn main() {
    let window = Window::new("App", (600, 400), 2)
        .with_bg(Color::rgb(0.15, 0.01, 0.06));

    let rect = Col::red(Rect::new((31, 31), (12, 12)));
    let img = Pos::right_bot(4, 4, Image::new("./data/1.png", window.render()));
    let text = Col::green("jie adding the КАПС README.,!:;|*@ Привет, мир ЪЁь!1230");

    let style = FontStyle::new().shadow((0, -1), Color::blue());
    let cyr = Font::from_style(Col::red("`ж`эьмъйы ЭЬМЪЙЫ"), style);
    let cyr = Pos::right(0, cyr);

    let hello = Font::new("Hello, world!")
        .monospaced()
        .shadow((1, -1), Color::rgb(0.6, 0.0, 0.7));

    window.run(move |render| {
        render.draw(&rect);
        render.draw(&img);
        render.draw(&Pos::left_top(0, 0, &text));
        render.draw(&hello);
        render.draw(&cyr);
    });
}
