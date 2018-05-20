extern crate glium;

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let mut events_loop = glium::glutin::EventsLoop::new();
    // 2. Parameters for building the Window.
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let context = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut frame = display.draw();
    loop {
    }
}
