#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{generate_joint, rotate, Chain};

use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Forward Kinematics Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let mut circle1 = generate_joint(-0.5, -0.4, "1.0", "0.0", "0.0", &display);
    let circle2 = generate_joint(
        circle1.middle.position[0],
        circle1.middle.position[1],
        "0.0",
        "1.0",
        "0.0",
        &display,
    );

    // amount of turns available in both directions
    let (mut _left_, mut _right_) = (0, 36);

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::Right) => {
                                if _right_ > 0 {
                                    circle1.vertex_buffer = rotate(-5.0, &mut circle1, &display);
                                    _left_ += 1;
                                    _right_ -= 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::Left) => {
                                if _left_ > 0 {
                                    circle1.vertex_buffer = rotate(5.0, &mut circle1, &display);
                                    _left_ -= 1;
                                    _right_ += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
        let mut frame = display.draw();

        // set canvas color
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        frame
            .draw(
                &circle1.vertex_buffer,
                &circle1.index_buffer,
                &circle1.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();

        draw(&mut frame, &mut circle1);
        frame.finish().unwrap();
    });
}

fn draw(frame: &mut glium::Frame, chain: &mut Chain) {
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
