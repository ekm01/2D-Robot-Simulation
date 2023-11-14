#[macro_use]
extern crate glium;

use glium::Surface;
use std::f32::consts;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();

    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Forward Kinematics Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let (mut circle_vertices, circle_indices) = generate_vertices();

    let mut circle_vertex_buffer = glium::VertexBuffer::new(&display, &circle_vertices).unwrap();

    let circle_index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TriangleFan,
        &circle_indices,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 330 core

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core

        out vec4 color;

        void main() {
            color = vec4(1.0, 1.0, 1.0, 1.0);
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

fn generate_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = vec![
        Vertex {
            position: [-0.25, 0.2], // top right
        },
        Vertex {
            position: [-0.35, -0.3], // bottom left
        },
        Vertex {
            position: [-0.35, 0.2], // top left
        },
    ];
    let mut indices: Vec<u16> = (0..=3).collect();

    // Generate vertices and indices for the circle
    let circle_segments = 100;
    let circle_radius = 0.05;
    for i in 3..=circle_segments {
        let theta = 2.0 * consts::PI * (i as f32) / (circle_segments as f32);
        let x = circle_radius * theta.cos();
        let y = circle_radius * theta.sin();
        vertices.push(Vertex {
            position: [x - 0.3, y - 0.3],
        });
        indices.push(i as u16);
    }

    (vertices, indices)
}
