#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{
    detect_collision, generate_chain, generate_claws, generate_object, gravity, rotate, Part,
    Vertex, DEF_HEIGHT, GROUND,
};

use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("2D Robot Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let mut chain1 = generate_chain(-0.5, -0.4, "1.0", "0.6", "0.0", &display);
    let mut chain2 = generate_chain(
        chain1.tip.position[0],
        chain1.tip.position[1],
        "0.0",
        "1.0",
        "0.0",
        &display,
    );
    let mut chain3 = generate_chain(
        chain2.tip.position[0],
        chain2.tip.position[1],
        "0.0",
        "0.0",
        "1.0",
        &display,
    );
    chain3.vertex_buffer = rotate(
        -90.0,
        &mut chain3,
        &display,
        chain2.tip.position[0],
        chain2.tip.position[1],
    );

    let vertex1 = Vertex {
        position: [0.15, -0.45], //bl
    };
    let vertex2 = Vertex {
        position: [0.2, -0.45], //br
    };
    let vertex3 = Vertex {
        position: [0.2, -0.35], //tr
    };
    let vertex4 = Vertex {
        position: [0.15, -0.35], //tl
    };

    let vertices = vec![vertex1, vertex2, vertex3, vertex4];

    let mut obj = generate_object(vertices, "0.0", "0.0", "0.0", &display);

    let (mut claw1, mut claw2) = generate_claws(chain3.tip, "1.0", "0.0", "0.0", &display);

    let (origin_x, origin_y) = (chain1.tip.position[0], chain1.tip.position[1] - DEF_HEIGHT);

    // amount of rotations left in both directions
    let (mut _left_chain1, mut _right_chain1) = (0, 30);
    let (mut _left_chain2, mut _right_chain2) = (0, 60);
    let (mut _left_chain3, mut _right_chain3) = (30, 24);
    let (mut _left_claw, mut _right_claw) = (0, 9);
    let mut _object = 0;

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        let chain2_x = chain1.tip.position[0];
                        let chain2_y = chain1.tip.position[1];

                        let chain3_x = chain2.tip.position[0];
                        let chain3_y = chain2.tip.position[1];

                        let claw1_x = claw1.tip.position[0];
                        let claw1_y = claw1.tip.position[1];

                        let claw2_x = claw2.tip.position[0];
                        let claw2_y = claw2.tip.position[1];

                        match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::Q) => {
                                if _left_chain1 > 0 {
                                    chain1.vertex_buffer =
                                        rotate(3.0, &mut chain1, &display, origin_x, origin_y);
                                    chain2.vertex_buffer =
                                        rotate(3.0, &mut chain2, &display, origin_x, origin_y);
                                    chain3.vertex_buffer =
                                        rotate(3.0, &mut chain3, &display, origin_x, origin_y);
                                    claw1.vertex_buffer =
                                        rotate(3.0, &mut claw1, &display, origin_x, origin_y);
                                    claw2.vertex_buffer =
                                        rotate(3.0, &mut claw2, &display, origin_x, origin_y);
                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(3.0, &mut obj, &display, origin_x, origin_y);
                                    }
                                    _left_chain1 -= 1;
                                    _right_chain1 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::W) => {
                                if _right_chain1 > 0
                                    && chain2.tip.position[1] >= origin_y
                                    && chain3.tip.position[1] >= origin_y
                                {
                                    chain1.vertex_buffer =
                                        rotate(-3.0, &mut chain1, &display, origin_x, origin_y);
                                    chain2.vertex_buffer =
                                        rotate(-3.0, &mut chain2, &display, origin_x, origin_y);
                                    chain3.vertex_buffer =
                                        rotate(-3.0, &mut chain3, &display, origin_x, origin_y);
                                    claw1.vertex_buffer =
                                        rotate(-3.0, &mut claw1, &display, origin_x, origin_y);
                                    claw2.vertex_buffer =
                                        rotate(-3.0, &mut claw2, &display, origin_x, origin_y);
                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(-3.0, &mut obj, &display, origin_x, origin_y);
                                    }

                                    _left_chain1 += 1;
                                    _right_chain1 -= 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::A) => {
                                if _left_chain2 > 0 {
                                    chain2.vertex_buffer =
                                        rotate(3.0, &mut chain2, &display, chain2_x, chain2_y);
                                    chain3.vertex_buffer =
                                        rotate(3.0, &mut chain3, &display, chain2_x, chain2_y);
                                    claw1.vertex_buffer =
                                        rotate(3.0, &mut claw1, &display, chain2_x, chain2_y);
                                    claw2.vertex_buffer =
                                        rotate(3.0, &mut claw2, &display, chain2_x, chain2_y);
                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(3.0, &mut obj, &display, chain2_x, chain2_y);
                                    }
                                    _left_chain2 -= 1;
                                    _right_chain2 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::S) => {
                                if _right_chain2 > 0
                                    && chain2.tip.position[1] >= origin_y
                                    && chain3.tip.position[1] >= origin_y
                                {
                                    chain2.vertex_buffer =
                                        rotate(-3.0, &mut chain2, &display, chain2_x, chain2_y);
                                    chain3.vertex_buffer =
                                        rotate(-3.0, &mut chain3, &display, chain2_x, chain2_y);
                                    claw1.vertex_buffer =
                                        rotate(-3.0, &mut claw1, &display, chain2_x, chain2_y);
                                    claw2.vertex_buffer =
                                        rotate(-3.0, &mut claw2, &display, chain2_x, chain2_y);
                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(-3.0, &mut obj, &display, chain2_x, chain2_y);
                                    }
                                    _left_chain2 += 1;
                                    _right_chain2 -= 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::Z) => {
                                if _left_chain3 > 0 {
                                    chain3.vertex_buffer =
                                        rotate(3.0, &mut chain3, &display, chain3_x, chain3_y);
                                    claw1.vertex_buffer =
                                        rotate(3.0, &mut claw1, &display, chain3_x, chain3_y);
                                    claw2.vertex_buffer =
                                        rotate(3.0, &mut claw2, &display, chain3_x, chain3_y);
                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(3.0, &mut obj, &display, chain3_x, chain3_y);
                                    }
                                    _left_chain3 -= 1;
                                    _right_chain3 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::X) => {
                                if _right_chain3 > 0 && chain3.tip.position[1] >= origin_y {
                                    chain3.vertex_buffer =
                                        rotate(-3.0, &mut chain3, &display, chain3_x, chain3_y);
                                    claw1.vertex_buffer =
                                        rotate(-3.0, &mut claw1, &display, chain3_x, chain3_y);
                                    claw2.vertex_buffer =
                                        rotate(-3.0, &mut claw2, &display, chain3_x, chain3_y);
                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(-3.0, &mut obj, &display, chain3_x, chain3_y);
                                    }
                                    _left_chain3 += 1;
                                    _right_chain3 -= 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::Key1) => {
                                if _left_claw > 0 {
                                    claw1.vertex_buffer =
                                        rotate(5.0, &mut claw1, &display, claw1_x, claw1_y);
                                    claw2.vertex_buffer =
                                        rotate(-5.0, &mut claw2, &display, claw2_x, claw2_y);
                                    if !detect_collision(&claw1, &claw2, &obj.vertices) {
                                        _object = 0;
                                    }

                                    _left_claw -= 1;
                                    _right_claw += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::Key2) => {
                                if _right_claw > 0 {
                                    claw1.vertex_buffer =
                                        rotate(-5.0, &mut claw1, &display, claw1_x, claw1_y);
                                    claw2.vertex_buffer =
                                        rotate(5.0, &mut claw2, &display, claw2_x, claw2_y);
                                    if detect_collision(&claw1, &claw2, &obj.vertices) {
                                        _object = 1;
                                    }
                                    _left_claw += 1;
                                    _right_claw -= 1;
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

        if _object == 0 {
            if obj.vertices[0].position[1] > GROUND {
                obj.vertex_buffer = gravity(&mut obj, &display);
                draw(&mut frame, &mut obj);
            }
        }

        // draw chains
        draw(&mut frame, &mut chain1);
        draw(&mut frame, &mut chain2);
        draw(&mut frame, &mut chain3);
        draw(&mut frame, &mut obj);
        draw(&mut frame, &mut claw1);
        draw(&mut frame, &mut claw2);

        frame.finish().unwrap();
    });
}

fn draw(frame: &mut glium::Frame, chain: &mut dyn Part) {
    frame
        .draw(
            chain.get_vertex_buf(),
            chain.get_index_buf(),
            chain.get_program(),
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}
