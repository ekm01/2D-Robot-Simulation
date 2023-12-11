#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{
    apply_gravity, create, detect_collisions, execute, generate_random_object, rotate, rotate_all,
    Part, State, DEF_HEIGHT,
};

use glium::{glutin::surface::WindowSurface, Surface};
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let primary_monitor = event_loop.available_monitors().next().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("2D Robot Simulation")
        .with_inner_size(primary_monitor.size().width, primary_monitor.size().height)
        .build(&event_loop);

    let display: &'static glium::Display<WindowSurface> = Box::leak(Box::new(display));
    let (mut parts, mut objects, dummy) = create(&display);
    let mut dummy: Box<dyn Part> = Box::new(dummy);
    let (origin_x, origin_y) = (
        parts.get_mut("chain1").unwrap().get_tip().unwrap().position[0],
        parts.get_mut("chain1").unwrap().get_tip().unwrap().position[1] - DEF_HEIGHT,
    );

    // amount of rotations left in both directions
    let (mut _left_chain1, mut _right_chain1) = (0, 30);
    let (mut _left_chain2, mut _right_chain2) = (0, 60);
    let (mut _left_chain3, mut _right_chain3) = (30, 24);
    let (mut _left_claw, mut _right_claw) = (0, 9);
    let mut _object = 0;
    let mut _base = (0, 0, 0, 0);
    let mut _state = (0, 0, 0, 0);

    // teach and jobs
    let mut jobs: Vec<State> = Vec::new();
    let mut state = None;
    let mut base_state = Some(State {
        l1: _left_chain1,
        l2: _left_chain2,
        l3: _left_chain3,
        l4: _left_claw,
        moved_object: String::new(),
    });

    // object to be moved
    let mut moved_object = String::new();
    let mut object_id = 0;

    // rng
    let mut rng = rand::thread_rng();

    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw();

        // set canvas color
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        // rotation points
        let chain2_x = parts.get_mut("chain1").unwrap().get_tip().unwrap().position[0];
        let chain2_y = parts.get_mut("chain1").unwrap().get_tip().unwrap().position[1];

        let chain3_x = parts.get_mut("chain2").unwrap().get_tip().unwrap().position[0];
        let chain3_y = parts.get_mut("chain2").unwrap().get_tip().unwrap().position[1];

        let claw1_x = parts.get_mut("claw1").unwrap().get_tip().unwrap().position[0];
        let claw1_y = parts.get_mut("claw1").unwrap().get_tip().unwrap().position[1];

        let claw2_x = parts.get_mut("claw2").unwrap().get_tip().unwrap().position[0];
        let claw2_y = parts.get_mut("claw2").unwrap().get_tip().unwrap().position[1];

        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == winit::event::ElementState::Pressed {
                        match input.virtual_keycode {
                            Some(winit::event::VirtualKeyCode::C) => {
                                let random_x: f32 = rng.gen_range(-0.2..=0.75);
                                let random_y: f32 = rng.gen_range(-0.43..=0.75);

                                let random_r: f32 = rng.gen_range(0.0..=1.0);

                                let random_g: f32 = rng.gen_range(0.0..=1.0);

                                let random_b: f32 = rng.gen_range(0.0..=1.0);

                                let random_object: Box<dyn Part> =
                                    Box::new(generate_random_object(
                                        (random_x, random_y),
                                        random_r.to_string().as_str(),
                                        random_g.to_string().as_str(),
                                        random_b.to_string().as_str(),
                                        &display,
                                    ));

                                let name = String::from("obj");
                                objects
                                    .insert(name + object_id.to_string().as_str(), random_object);
                                object_id += 1;
                            }
                            Some(winit::event::VirtualKeyCode::R) => {
                                let name =
                                    String::from("obj") + (object_id - 1).to_string().as_str();
                                if objects.get_mut(name.as_str()).is_some() {
                                    objects.remove(name.as_str());
                                    object_id -= 1;
                                }
                            }

                            Some(winit::event::VirtualKeyCode::T) => {
                                let mut state_object = String::new();
                                state_object = state_object.to_string() + moved_object.as_str();
                                let state = State {
                                    l1: _left_chain1,
                                    l2: _left_chain2,
                                    l3: _left_chain3,
                                    l4: _left_claw,
                                    moved_object: state_object,
                                };
                                jobs.insert(0, state);
                            }
                            Some(winit::event::VirtualKeyCode::E) => {
                                state = jobs.pop();
                                if state.is_some() {
                                    _state = (1, 0, 0, 0);
                                }
                            }
                            Some(winit::event::VirtualKeyCode::L) => {
                                _object = 0;
                            }
                            Some(winit::event::VirtualKeyCode::B) => {
                                _base = (1, 0, 0, 0);
                            }
                            Some(winit::event::VirtualKeyCode::Q) => {
                                if _left_chain1 > 0 {
                                    rotate_all(3.0, &mut parts, &display, origin_x, origin_y);

                                    if _object == 1 {
                                        let obj_vertex_buf = rotate(
                                            3.0,
                                            objects
                                                .get_mut(moved_object.as_str())
                                                .unwrap()
                                                .as_mut(),
                                            &display,
                                            origin_x,
                                            origin_y,
                                        );
                                        objects
                                            .get_mut(moved_object.as_str())
                                            .unwrap()
                                            .set_vertex_buf(obj_vertex_buf);
                                    }
                                    _left_chain1 -= 1;
                                    _right_chain1 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::W) => {
                                if _right_chain1 > 0 {
                                    rotate_all(-3.0, &mut parts, &display, origin_x, origin_y);

                                    if _object == 1 {
                                        let obj_vertex_buf = rotate(
                                            -3.0,
                                            objects
                                                .get_mut(moved_object.as_str())
                                                .unwrap()
                                                .as_mut(),
                                            &display,
                                            origin_x,
                                            origin_y,
                                        );
                                        objects
                                            .get_mut(moved_object.as_str())
                                            .unwrap()
                                            .set_vertex_buf(obj_vertex_buf);
                                    }

                                    _left_chain1 += 1;
                                    _right_chain1 -= 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::A) => {
                                if _left_chain2 > 0 {
                                    let chain1 = parts.remove("chain1").unwrap();
                                    rotate_all(3.0, &mut parts, &display, chain2_x, chain2_y);
                                    parts.insert("chain1".to_string(), chain1);

                                    if _object == 1 {
                                        let obj_vertex_buf = rotate(
                                            3.0,
                                            objects
                                                .get_mut(moved_object.as_str())
                                                .unwrap()
                                                .as_mut(),
                                            &display,
                                            chain2_x,
                                            chain2_y,
                                        );
                                        objects
                                            .get_mut(moved_object.as_str())
                                            .unwrap()
                                            .set_vertex_buf(obj_vertex_buf);
                                    }
                                    _left_chain2 -= 1;
                                    _right_chain2 += 1;
                                }
                            }
                            Some(winit::event::VirtualKeyCode::S) => {
                                if _right_chain2 > 0 {
                                    let chain1 = parts.remove("chain1").unwrap();
                                    rotate_all(-3.0, &mut parts, &display, chain2_x, chain2_y);
                                    parts.insert("chain1".to_string(), chain1);

                                    if _object == 1 {
                                        let obj_vertex_buf = rotate(
                                            -3.0,
                                            objects
                                                .get_mut(moved_object.as_str())
                                                .unwrap()
                                                .as_mut(),
                                            &display,
                                            chain2_x,
                                            chain2_y,
                                        );
                                        objects
                                            .get_mut(moved_object.as_str())
                                            .unwrap()
                                            .set_vertex_buf(obj_vertex_buf);
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
                                    parts.insert("chain1".to_string(), chain1);
                                    parts.insert("chain2".to_string(), chain2);

                                    if _object == 1 {
                                        let obj_vertex_buf = rotate(
                                            3.0,
                                            objects
                                                .get_mut(moved_object.as_str())
                                                .unwrap()
                                                .as_mut(),
                                            &display,
                                            chain3_x,
                                            chain3_y,
                                        );
                                        objects
                                            .get_mut(moved_object.as_str())
                                            .unwrap()
                                            .set_vertex_buf(obj_vertex_buf);
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
                                    parts.insert("chain1".to_string(), chain1);
                                    parts.insert("chain2".to_string(), chain2);

                                    if _object == 1 {
                                        let obj_vertex_buf = rotate(
                                            -3.0,
                                            objects
                                                .get_mut(moved_object.as_str())
                                                .unwrap()
                                                .as_mut(),
                                            &display,
                                            chain3_x,
                                            chain3_y,
                                        );
                                        objects
                                            .get_mut(moved_object.as_str())
                                            .unwrap()
                                            .set_vertex_buf(obj_vertex_buf);
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

                                    if !detect_collisions(
                                        parts.get("claw1").unwrap().as_ref(),
                                        parts.get("claw2").unwrap().as_ref(),
                                        &mut objects,
                                    )
                                    .is_some()
                                    {
                                        _object = 0;
                                        moved_object = String::new();
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
                                    let coll_option = detect_collisions(
                                        parts.get("claw1").unwrap().as_ref(),
                                        parts.get("claw2").unwrap().as_ref(),
                                        &mut objects,
                                    );

                                    if coll_option.is_some() {
                                        _object = 1;
                                        moved_object = coll_option.unwrap();
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

        apply_gravity(
            moved_object.as_str(),
            state.as_mut(),
            &mut objects,
            &display,
        );

        let base_value = base_state.as_mut();

        let mut temp = dummy.as_mut();

        if base_value.is_some() {
            execute(
                (&mut _left_chain1, &mut _right_chain1),
                (&mut _left_chain2, &mut _right_chain2),
                (&mut _left_chain3, &mut _right_chain3),
                (&mut _left_claw, &mut _right_claw),
                &mut _base,
                (origin_x, origin_y),
                (chain2_x, chain2_y),
                (chain3_x, chain3_y),
                (claw1_x, claw1_y),
                (claw2_x, claw2_y),
                &mut parts,
                &display,
                base_value.unwrap(),
                temp,
                &mut _object,
            );
        }

        let state_value = state.as_mut();
        if state_value.is_some() {
            if state_value.as_ref().unwrap().moved_object != String::new() {
                temp = objects
                    .get_mut(state_value.as_ref().unwrap().moved_object.as_str())
                    .unwrap()
                    .as_mut();
            }
            execute(
                (&mut _left_chain1, &mut _right_chain1),
                (&mut _left_chain2, &mut _right_chain2),
                (&mut _left_chain3, &mut _right_chain3),
                (&mut _left_claw, &mut _right_claw),
                &mut _state,
                (origin_x, origin_y),
                (chain2_x, chain2_y),
                (chain3_x, chain3_y),
                (claw1_x, claw1_y),
                (claw2_x, claw2_y),
                &mut parts,
                &display,
                state_value.unwrap(),
                temp,
                &mut _object,
            );
        }

        // draw objects, then chains
        draw_objects(&mut frame, &mut objects);
        draw_chains(&mut frame, &mut parts);

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

fn draw_chains(frame: &mut glium::Frame, chains: &mut HashMap<String, Box<dyn Part>>) {
    draw(frame, chains.get_mut("chain1").unwrap().as_mut());
    draw(frame, chains.get_mut("chain2").unwrap().as_mut());
    draw(frame, chains.get_mut("chain3").unwrap().as_mut());
    draw(frame, chains.get_mut("claw1").unwrap().as_mut());
    draw(frame, chains.get_mut("claw2").unwrap().as_mut());
}

fn draw_objects(frame: &mut glium::Frame, parts: &mut HashMap<String, Box<dyn Part>>) {
    for (_, part) in parts.iter_mut() {
        frame
            .draw(
                part.get_vertex_buf(),
                part.get_index_buf(),
                part.get_program(),
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }
}
