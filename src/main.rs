#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{generate_program, generate_vertices};

use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Forward Kinematics Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let (mut circle_vertices, circle_indices, middle) = generate_vertices(-0.3, -0.3);

    let mut circle_vertex_buffer = glium::VertexBuffer::new(&display, &circle_vertices).unwrap();

    let circle_index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleFan,
        &circle_indices,
    )
    .unwrap();

    let program = generate_program("1.0", "0.0", "0.0", &display);
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed
                        && input.virtual_keycode == Some(winit::event::VirtualKeyCode::Left)
                    {
                        let rotation_angle = -45.0_f32.to_radians();
                        let rotation_matrix = [
                            [rotation_angle.cos(), -rotation_angle.sin()],
                            [rotation_angle.sin(), rotation_angle.cos()],
                        ];
                        for vertex in &mut circle_vertices {
                            let x = vertex.position[0] + 0.3; // Adjust the value based on your needs
                            let y = vertex.position[1] + 0.3;

                            vertex.position[0] =
                                rotation_matrix[0][0] * x + rotation_matrix[0][1] * y - 0.3;
                            vertex.position[1] =
                                rotation_matrix[1][0] * x + rotation_matrix[1][1] * y - 0.3;
                        }
                        circle_vertex_buffer =
                            glium::VertexBuffer::new(&display, &circle_vertices).unwrap();
                    }
                }
                _ => (),
            },
            _ => (),
        }
        let mut target = display.draw();

        // set canvas color
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target
            .draw(
                &circle_vertex_buffer,
                &circle_index_buffer,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
