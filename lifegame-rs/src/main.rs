
extern crate glium;

//#[derive(Copy, Clone)]
//struct MyVertex {
//    position: [f32; 3],
//    texcoords: [f32; 2],
//}
//
//// you must pass the list of members to the macro
//implement_vertex!(MyVertex, position, texcoords);
//
//fn main() {
//    // 1. The **winit::EventsLoop** for handling events.
//    let mut events_loop = glium::glutin::EventsLoop::new();
//    // 2. Parameters for building the Window.
//    let window = glium::glutin::WindowBuilder::new()
//        .with_dimensions(1024, 768)
//        .with_title("Hello world");
//    // 3. Parameters for building the OpenGL context.
//    let context = glium::glutin::ContextBuilder::new();
//    // 4. Build the Display with the given window and OpenGL context parameters and register the
//    //    window with the events_loop.
//    let display = glium::Display::new(window, context, &events_loop).unwrap();
//
//    // drawing with a single vertex buffer
//    let mut frame = display.draw();
//
//    let indices = PointsList;
//
//    let data = &[
//        MyVertex {
//            position: [0.0, 0.0, 0.4],
//            texcoords: [0.0, 1.0]
//        },
//        MyVertex {
//            position: [12.0, 4.5, -1.8],
//            texcoords: [1.0, 0.5]
//        },
//        MyVertex {
//            position: [-7.124, 0.1, 0.0],
//            texcoords: [0.0, 0.4]
//        },
//    ];
//    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, data);
//    frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
//
//
//
//    loop {
//    }
//}
fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
