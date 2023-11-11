#[macro_use]
extern crate glium;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    // We start by creating the EventLoop, this can only be done once per process.
    // This also needs to happen on the main thread to make the program portable.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Forward Kinematics Simulation")
        .build(&event_loop);

    let monitor_handle = _window.available_monitors().next().unwrap();
    let fs = winit::window::Fullscreen::Borderless(Some(monitor_handle));
    _window.set_fullscreen(Some(fs));

    let mut shape = vec![
        Vertex {
            position: [-0.5, 0.5],
        },
        Vertex {
            position: [0.5, 0.5],
        },
        Vertex {
            position: [-0.5, -0.5],
        },
        Vertex {
            position: [0.5, -0.5],
        },
    ];
    let mut vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed
                        && input.virtual_keycode == Some(winit::event::VirtualKeyCode::Right)
                    {
                        let rotation_angle = 45.0_f32.to_radians();
                        let rotation_matrix = [
                            [rotation_angle.cos(), -rotation_angle.sin()],
                            [rotation_angle.sin(), rotation_angle.cos()],
                        ];
                        for vertex in &mut shape {
                            let x = vertex.position[0]; // Adjust the value based on your needs
                            let y = vertex.position[1];
                            vertex.position[0] =
                                rotation_matrix[0][0] * x + rotation_matrix[0][1] * y;
                            vertex.position[1] =
                                rotation_matrix[1][0] * x + rotation_matrix[1][1] * y;
                        }
                        vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
                    }
                }
                _ => (),
            },
            _ => (),
        }
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
