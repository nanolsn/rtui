# RTUI
UI implementation for RT engine. Based on the OpenGL (temporary, I hope).

## Example
```rust
fn main() {
    // Make the Window with background color.
    let window = Window::new("App", (600, 400), 2)
        .with_bg(Color::rgb(0.15, 0.01, 0.06));

    // Make an image, positioned to right bottom window corner with padding 4.
    let img = Pos::right_bot(4, 4, Image::new("./data/1.png", window.render()));

    // Make the `Hello, world!` text on the screen, with the monospaced style and the shadow.
    let hello = Font::new("Hello, world!")
        .monospaced()
        .shadow((1, -1), Color::rgb(0.6, 0.0, 0.7));

    // Run the game loop.
    window.run(move |render| {
        render.draw(&img);
        render.draw(&hello);
    });
}
```
