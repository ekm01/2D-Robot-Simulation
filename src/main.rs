#[macro_use]
extern crate glium;

mod robot;

use robot::robot::{
    generate_chain, generate_claws, generate_object, rotate, Claw, Part, Vertex, DEF_HEIGHT,
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
        position: [0.15, -0.5], //bl
    };
    let vertex2 = Vertex {
        position: [0.2, -0.5], //br
    };
    let vertex3 = Vertex {
        position: [0.2, -0.4], //tr
    };
    let vertex4 = Vertex {
        position: [0.15, -0.4], //tl
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

        // draw chains
        draw(&mut frame, &mut obj);
        draw(&mut frame, &mut chain1);
        draw(&mut frame, &mut chain2);
        draw(&mut frame, &mut chain3);
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

fn check_boundaries(ray_start: &Vertex, first: &Vertex, second: &Vertex) -> bool {
    let (x, y) = (ray_start.position[0], ray_start.position[1]);
    let (x1, y1) = (first.position[0], first.position[1]);
    let (x2, y2) = (second.position[0], second.position[1]);

    if ((y1 <= y && y < y2) || (y2 <= y && y < y1)) && x < f32::max(x1, x2) {
        return true;
    }
    false
}

fn ray_edge_intersect(ray_start: &Vertex, first: &Vertex, second: &Vertex) -> f32 {
    let (x, y) = (ray_start.position[0], ray_start.position[1]);
    let (x1, y1) = (first.position[0], first.position[1]);
    let (x2, y2) = (second.position[0], second.position[1]);

    let scaling_factor = ((x1 - x) * (y2 - y1) - (y1 - y) * (x2 - x1)) / (y2 - y1);

    x + scaling_factor + 0.07
}

fn detect_collision(claw1: &Claw, claw2: &Claw, vertices: &Vec<Vertex>) -> bool {
    let mut res = false;
    let length = vertices.len();

    // number of intersections
    let (mut num_intersec_right, mut num_intersec_left) = (0, 0);
    for i in 0..length {
        // get all the edges in a cyclic manner
        let first = vertices[i];
        let second = vertices[(i + 1) % length];

        if check_boundaries(&claw1.vertices[2], &first, &second)
            && check_boundaries(&claw2.vertices[2], &first, &second)
        {
            let intersec_right = ray_edge_intersect(&claw1.vertices[2], &first, &second);
            let intersec_left = ray_edge_intersect(&claw2.vertices[2], &first, &second);

            if claw1.vertices[2].position[0] < intersec_right {
                num_intersec_right += 1;
            }

            if claw2.vertices[2].position[0] < intersec_left {
                num_intersec_left += 1;
            }
        }
    }

    if num_intersec_left % 2 == 1 && num_intersec_right % 2 == 1 {
        res = true;
    }

    res
}
