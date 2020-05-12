use super::render::Render;
use crate::{
    common::{
        Color,
        Vec2d,
    },
};

pub struct Window {
    context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    event_loop: glutin::event_loop::EventLoop<()>,
    render: Render,
    bg: Color,
}

impl Window {
    pub fn new<S>(title: S, (w, h): (u32, u32), pixel_size: i32) -> Self
        where
            S: Into<String>,
    {
        let event_loop = glutin::event_loop::EventLoop::new();

        let size = glutin::dpi::LogicalSize::new(w, h);
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title(title);

        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_gl_profile(glutin::GlProfile::Core)
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .build_windowed(wb, &event_loop)
            .unwrap();

        let context = unsafe {
            context.make_current().unwrap()
        };

        let render = Render::new(&context, pixel_size).unwrap();

        Window {
            context,
            event_loop,
            render,
            bg: Color::default(),
        }
    }

    pub fn render(&self) -> &Render { &self.render }

    pub fn with_bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    pub fn run<F>(self, mut draw_frame_fn: F)
        where
            F: FnMut(&mut Render) + 'static,
    {
        let mut render = self.render;
        let mut focused = true;
        let context = self.context;
        let bg = self.bg;

        self.event_loop.run(move |event, _, control_flow| {
            use glutin::{
                event::{Event, WindowEvent, StartCause},
                event_loop::ControlFlow,
            };

            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        let (w, h) = size.into();
                        render.resize(Vec2d::new(w, h));
                        context.resize(size);
                    }
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit }
                    WindowEvent::Focused(flag) => { focused = flag }
                    _ => (),
                }
                Event::NewEvents(StartCause::Poll) => {
                    if !focused { return; }

                    render.begin_draw_frame();
                    render.clear(bg);

                    draw_frame_fn(&mut render);

                    render.end_draw_frame();

                    context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
