#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{generate_joint, rotate, Chain, DEF_HEIGHT};

use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Forward Kinematics Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let mut chain1 = generate_joint(-0.5, -0.4, "1.0", "0.0", "0.0", &display);
    let mut chain2 = generate_joint(
        chain1.middle.position[0],
        chain1.middle.position[1],
        "0.0",
        "1.0",
        "0.0",
        &display,
    );

    // amount of turns available in both directions
    let (mut _left_chain1, mut _right_chain1) = (0, 18);
    let (mut _left_chain2, mut _right_chain2) = (0, 31);

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        let chain1_x = chain1.middle.position[0];
                        let chain1_y = chain1.middle.position[1] - DEF_HEIGHT;

                        let chain2_x = chain2.middle.position[0];
                        let chain2_y = chain2.middle.position[1] - DEF_HEIGHT;

                        match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::W) => {
                                if _right_chain1 > 0 {
                                    chain1.vertex_buffer =
                                        rotate(-5.0, &mut chain1, &display, chain1_x, chain1_y);
                                    chain2.vertex_buffer =
                                        rotate(-5.0, &mut chain2, &display, chain1_x, chain1_y);
                                    _left_chain1 += 1;
                                    _right_chain1 -= 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::Q) => {
                                if _left_chain1 > 0 {
                                    chain1.vertex_buffer =
                                        rotate(5.0, &mut chain1, &display, chain1_x, chain1_y);
                                    chain2.vertex_buffer =
                                        rotate(5.0, &mut chain2, &display, chain1_x, chain1_y);
                                    _left_chain1 -= 1;
                                    _right_chain1 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::A) => {
                                if _left_chain2 > 0 {
                                    chain2.vertex_buffer =
                                        rotate(5.0, &mut chain2, &display, chain2_x, chain2_y);
                                    _left_chain2 -= 1;
                                    _right_chain2 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::S) => {
                                if _right_chain2 > 0 {
                                    chain2.vertex_buffer =
                                        rotate(-5.0, &mut chain2, &display, chain2_x, chain2_y);
                                    _left_chain2 += 1;
                                    _right_chain2 -= 1;
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

        // draw chains
        draw(&mut frame, &mut chain1);
        draw(&mut frame, &mut chain2);
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
