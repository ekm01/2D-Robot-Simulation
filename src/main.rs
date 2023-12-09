#[macro_use]
extern crate glium;

mod robot;

use std::collections::HashMap;

use robot::robot::{
    apply_gravity, detect_collision, generate_chain, generate_claws, generate_object, rotate,
    rotate_all, Part, Vertex, DEF_HEIGHT, GROUND,
};

use glium::Surface;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("2D Robot Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let mut chain1: Box<dyn Part> =
        Box::new(generate_chain(-0.5, -0.4, "1.0", "0.6", "0.0", &display));
    let mut chain2: Box<dyn Part> = Box::new(generate_chain(
        chain1.get_tip().unwrap().position[0],
        chain1.get_tip().unwrap().position[1],
        "0.0",
        "1.0",
        "0.0",
        &display,
    ));
    let mut chain3: Box<dyn Part> = Box::new(generate_chain(
        chain2.get_tip().unwrap().position[0],
        chain2.get_tip().unwrap().position[1],
        "0.0",
        "0.0",
        "1.0",
        &display,
    ));
    let chain3_buf = rotate(
        -90.0,
        chain3.as_mut(),
        &display,
        chain2.get_tip().unwrap().position[0],
        chain2.get_tip().unwrap().position[1],
    );

    chain3.set_vertex_buf(chain3_buf);

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

    let (claw1, claw2) = generate_claws(*chain3.get_tip().unwrap(), "1.0", "0.0", "0.0", &display);

    let (origin_x, origin_y) = (
        chain1.get_tip().unwrap().position[0],
        chain1.get_tip().unwrap().position[1] - DEF_HEIGHT,
    );

    // amount of rotations left in both directions
    let (mut _left_chain1, mut _right_chain1) = (0, 30);
    let (mut _left_chain2, mut _right_chain2) = (0, 60);
    let (mut _left_chain3, mut _right_chain3) = (30, 24);
    let (mut _left_claw, mut _right_claw) = (0, 9);
    let mut _object = 0;
    let mut _base = 0;

    let mut parts: HashMap<&str, Box<dyn Part>> = HashMap::new();

    parts.insert("chain1", chain1);
    parts.insert("chain2", chain2);
    parts.insert("chain3", chain3);
    parts.insert("claw1", claw1);
    parts.insert("claw2", claw2);

    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw();

        // set canvas color
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        let chain2_x =
                            parts.get_mut("chain1").unwrap().get_tip().unwrap().position[0];
                        let chain2_y =
                            parts.get_mut("chain1").unwrap().get_tip().unwrap().position[1];

                        let chain3_x =
                            parts.get_mut("chain2").unwrap().get_tip().unwrap().position[0];
                        let chain3_y =
                            parts.get_mut("chain2").unwrap().get_tip().unwrap().position[1];

                        let claw1_x =
                            parts.get_mut("claw1").unwrap().get_tip().unwrap().position[0];
                        let claw1_y =
                            parts.get_mut("claw1").unwrap().get_tip().unwrap().position[1];

                        let claw2_x =
                            parts.get_mut("claw2").unwrap().get_tip().unwrap().position[0];
                        let claw2_y =
                            parts.get_mut("claw2").unwrap().get_tip().unwrap().position[1];

                        match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::B) => {
                                _base = 1;
                            }
                            Some(winit::event::VirtualKeyCode::Q) => {
                                if _left_chain1 > 0 {
                                    rotate_all(3.0, &mut parts, &display, origin_x, origin_y);

                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(3.0, &mut obj, &display, origin_x, origin_y);
                                    }
                                    _left_chain1 -= 1;
                                    _right_chain1 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::W) => {
                                if _right_chain1 > 0 {
                                    rotate_all(-3.0, &mut parts, &display, origin_x, origin_y);

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
                                    let chain1 = parts.remove("chain1").unwrap();
                                    rotate_all(3.0, &mut parts, &display, chain2_x, chain2_y);
                                    parts.insert("chain1", chain1);

                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(3.0, &mut obj, &display, chain2_x, chain2_y);
                                    }
                                    _left_chain2 -= 1;
                                    _right_chain2 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::S) => {
                                if _right_chain2 > 0 {
                                    let chain1 = parts.remove("chain1").unwrap();
                                    rotate_all(-3.0, &mut parts, &display, chain2_x, chain2_y);
                                    parts.insert("chain1", chain1);

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
                                    let chain1 = parts.remove("chain1").unwrap();
                                    let chain2 = parts.remove("chain2").unwrap();
                                    rotate_all(3.0, &mut parts, &display, chain3_x, chain3_y);
                                    parts.insert("chain1", chain1);
                                    parts.insert("chain2", chain2);

                                    if _object == 1 {
                                        obj.vertex_buffer =
                                            rotate(3.0, &mut obj, &display, chain3_x, chain3_y);
                                    }
                                    _left_chain3 -= 1;
                                    _right_chain3 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::X) => {
                                if _right_chain3 > 0 {
                                    let chain1 = parts.remove("chain1").unwrap();
                                    let chain2 = parts.remove("chain2").unwrap();
                                    rotate_all(-3.0, &mut parts, &display, chain3_x, chain3_y);
                                    parts.insert("chain1", chain1);
                                    parts.insert("chain2", chain2);

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
                                    let claw1_vb = rotate(
                                        5.0,
                                        parts.get_mut("claw1").unwrap().as_mut(),
                                        &display,
                                        claw1_x,
                                        claw1_y,
                                    );

                                    let claw2_vb = rotate(
                                        -5.0,
                                        parts.get_mut("claw2").unwrap().as_mut(),
                                        &display,
                                        claw2_x,
                                        claw2_y,
                                    );

                                    parts.get_mut("claw1").unwrap().set_vertex_buf(claw1_vb);
                                    parts.get_mut("claw2").unwrap().set_vertex_buf(claw2_vb);

                                    if !detect_collision(
                                        parts.get("claw1").unwrap().as_ref(),
                                        parts.get("claw2").unwrap().as_ref(),
                                        &obj.vertices,
                                    ) {
                                        _object = 0;
                                    }

                                    _left_claw -= 1;
                                    _right_claw += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::Key2) => {
                                if _right_claw > 0 {
                                    let claw1_vb = rotate(
                                        -5.0,
                                        parts.get_mut("claw1").unwrap().as_mut(),
                                        &display,
                                        claw1_x,
                                        claw1_y,
                                    );

                                    let claw2_vb = rotate(
                                        5.0,
                                        parts.get_mut("claw2").unwrap().as_mut(),
                                        &display,
                                        claw2_x,
                                        claw2_y,
                                    );

                                    parts.get_mut("claw1").unwrap().set_vertex_buf(claw1_vb);
                                    parts.get_mut("claw2").unwrap().set_vertex_buf(claw2_vb);

                                    if detect_collision(
                                        parts.get("claw1").unwrap().as_ref(),
                                        parts.get("claw2").unwrap().as_ref(),
                                        &obj.vertices,
                                    ) {
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

        if _object == 0 {
            if obj.vertices[0].position[1] > GROUND {
                obj.vertex_buffer = apply_gravity(&mut obj, &display);
            }
        }

        if _base == 1 && _left_chain1 != 0 {
            rotate_all(3.0, &mut parts, &display, origin_x, origin_y);
            _left_chain1 -= 1;
            if _left_chain1 == 0 {
                _base = 0;
            }
            _right_chain1 += 1;
        }

        // draw chains
        draw(&mut frame, parts.get_mut("chain1").unwrap().as_mut());
        draw(&mut frame, parts.get_mut("chain2").unwrap().as_mut());
        draw(&mut frame, parts.get_mut("chain3").unwrap().as_mut());
        draw(&mut frame, &mut obj);
        draw(&mut frame, parts.get_mut("claw1").unwrap().as_mut());
        draw(&mut frame, parts.get_mut("claw2").unwrap().as_mut());

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
