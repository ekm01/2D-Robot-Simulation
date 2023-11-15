#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{generate_joint, Chain};

use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Forward Kinematics Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let mut circle1 = generate_joint(-0.3, -0.3, "1.0", "0.0", "0.0", &display);
    let circle2 = generate_joint(
        circle1.middle.position[0],
        circle1.middle.position[1],
        "0.0",
        "1.0",
        "0.0",
        &display,
    );

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
                        /*match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::Q) => exp,
                            None => expr,
                        }*/
                        let rotation_angle = -10.0_f32.to_radians();
                        let rotation_matrix = [
                            [rotation_angle.cos(), -rotation_angle.sin()],
                            [rotation_angle.sin(), rotation_angle.cos()],
                        ];
                        for vertex in &mut circle1.vertices {
                            let x = vertex.position[0] + 0.3; // Adjust the value based on your needs
                            let y = vertex.position[1] + 0.3;

                            vertex.position[0] =
                                rotation_matrix[0][0] * x + rotation_matrix[0][1] * y - 0.3;
                            vertex.position[1] =
                                rotation_matrix[1][0] * x + rotation_matrix[1][1] * y - 0.3;
                        }
                        circle1.vertex_buffer =
                            glium::VertexBuffer::new(&display, &circle1.vertices).unwrap();
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
                &circle1.vertex_buffer,
                &circle1.index_buffer,
                &circle1.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();

        drawido(&mut target, &mut circle1);
        target.finish().unwrap();
    });
}

fn drawido(frame: &mut glium::Frame, chain: &mut Chain) {
    frame
        .draw(
            &chain.vertex_buffer,
            &chain.index_buffer,
            &chain.program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}
