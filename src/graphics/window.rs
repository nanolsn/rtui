use super::render::Render;
use crate::{
    common::Color,
    ui::root::Root,
};

pub struct Window {
    pub(super) context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    event_loop: glutin::event_loop::EventLoop<()>,
    render: Render,
    root: Root,
}

impl Window {
    pub fn new<S>(title: S, (w, h): (u32, u32)) -> Self
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

        let render = Render::new(&context);

        let root = Root::new(Color::default());

        Window {
            context,
            event_loop,
            render,
            root,
        }
    }

    #[allow(dead_code)]
    pub fn set_bg(&mut self, color: Color) { self.root.bg = color }

    pub fn run<F>(self, mut f: F)
        where
            F: FnMut(&Render) + 'static,
    {
        let mut render = self.render;
        let mut focused = true;
        let context = self.context;
        let root = self.root;

        self.event_loop.run(move |event, _, control_flow| {
            use glutin::{
                event::{Event, WindowEvent, StartCause},
                event_loop::ControlFlow,
            };

            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        render.resize(size.into());
                        context.resize(size);
                    }
                    WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit }
                    WindowEvent::Focused(f) => { focused = f }
                    _ => (),
                }
                Event::NewEvents(StartCause::Poll) => {
                    if !focused { return; }

                    render.use_program();
                    render.clear(root.bg);

                    f(&render);

                    context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}
