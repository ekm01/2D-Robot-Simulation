pub mod robot {
    use glium::{glutin::surface::WindowSurface, VertexBuffer};
    use std::collections::HashMap;
    use std::f32::consts::PI;
    use std::thread;
    use std::time::Duration;

    #[derive(Copy, Clone, Debug)]
    pub struct Vertex {
        pub position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    const DEF_RADIUS: f32 = 0.05;
    const SLEEP_DURATION: Duration = Duration::from_millis(40);
    pub const DEF_THINNING: f32 = 0.02;
    pub const DEF_HEIGHT: f32 = 0.4;
    pub const GROUND: f32 = -0.43;

    pub struct State {
        pub l1: i32,
        pub l2: i32,
        pub l3: i32,
        pub l4: i32,
    }

    pub trait Part {
        fn get_vertex_buf(&self) -> &glium::VertexBuffer<Vertex>;
        fn set_vertex_buf(&mut self, value: VertexBuffer<Vertex>);
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32>;
        fn get_program(&self) -> &glium::program::Program;
        fn get_vertices(&mut self) -> &mut Vec<Vertex>;
        fn get_vertices_ref(&self) -> &Vec<Vertex>;
        fn get_tip(&mut self) -> Option<&mut Vertex>;
    }

    pub struct Chain {
        // link and joint
        pub vertices: Vec<Vertex>,
        pub indices: Vec<u32>,
        pub tip: Vertex,
        pub vertex_buffer: glium::VertexBuffer<Vertex>,
        pub index_buffer: glium::IndexBuffer<u32>,
        pub program: glium::program::Program,
    }

    impl Part for Chain {
        fn get_vertex_buf(&self) -> &glium::VertexBuffer<Vertex> {
            &self.vertex_buffer
        }
        fn set_vertex_buf(&mut self, value: VertexBuffer<Vertex>) {
            self.vertex_buffer = value;
        }
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32> {
            &self.index_buffer
        }
        fn get_program(&self) -> &glium::program::Program {
            &self.program
        }
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            &mut self.vertices
        }
        fn get_vertices_ref(&self) -> &Vec<Vertex> {
            &self.vertices
        }
        fn get_tip(&mut self) -> Option<&mut Vertex> {
            Some(&mut self.tip)
        }
    }

    pub struct Claw {
        pub vertices: Vec<Vertex>,
        pub tip: Vertex,
        pub vertex_buffer: glium::VertexBuffer<Vertex>,
        pub index_buffer: glium::IndexBuffer<u32>,
        pub program: glium::program::Program,
    }

    impl Part for Claw {
        fn get_vertex_buf(&self) -> &glium::VertexBuffer<Vertex> {
            &self.vertex_buffer
        }
        fn set_vertex_buf(&mut self, value: VertexBuffer<Vertex>) {
            self.vertex_buffer = value;
        }
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32> {
            &self.index_buffer
        }
        fn get_program(&self) -> &glium::program::Program {
            &self.program
        }
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            &mut self.vertices
        }
        fn get_vertices_ref(&self) -> &Vec<Vertex> {
            &self.vertices
        }
        fn get_tip(&mut self) -> Option<&mut Vertex> {
            Some(&mut self.tip)
        }
    }

    pub struct Object {
        pub vertices: Vec<Vertex>,
        pub vertex_buffer: glium::VertexBuffer<Vertex>,
        pub index_buffer: glium::IndexBuffer<u32>,
        pub program: glium::program::Program,
    }

    impl Part for Object {
        fn get_vertex_buf(&self) -> &glium::VertexBuffer<Vertex> {
            &self.vertex_buffer
        }
        fn set_vertex_buf(&mut self, value: VertexBuffer<Vertex>) {
            self.vertex_buffer = value;
        }
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32> {
            &self.index_buffer
        }
        fn get_program(&self) -> &glium::program::Program {
            &self.program
        }
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            &mut self.vertices
        }
        fn get_vertices_ref(&self) -> &Vec<Vertex> {
            &self.vertices
        }
        fn get_tip(&mut self) -> Option<&mut Vertex> {
            None
        }
    }

    pub fn generate_object(
        vertices: Vec<Vertex>,
        r: &str,
        g: &str,
        b: &str,
        disp: &glium::Display<WindowSurface>,
    ) -> Object {
        let indices = (0..=3).collect();
        let (vertex_buffer, index_buffer) = generate_vertex_index_buffer(disp, &vertices, &indices);
        let program = generate_program(r, g, b, disp);
        Object {
            vertices,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    pub fn generate_claws(
        vertex: Vertex,
        r: &str,
        g: &str,
        b: &str,
        disp: &glium::Display<WindowSurface>,
    ) -> (Box<dyn Part>, Box<dyn Part>) {
        let claw1_vertices = vec![
            Vertex {
                position: [
                    // root
                    vertex.position[0],
                    vertex.position[1] + DEF_THINNING + 0.01,
                ],
            },
            Vertex {
                position: [
                    // middle
                    vertex.position[0] + 0.01,
                    vertex.position[1] + DEF_THINNING + 0.04,
                ],
            },
            Vertex {
                position: [
                    // tip
                    vertex.position[0] + 0.06,
                    vertex.position[1] + DEF_THINNING + 0.03,
                ],
            },
        ];

        let claw2_vertices = vec![
            Vertex {
                position: [
                    // root
                    vertex.position[0],
                    vertex.position[1] - DEF_THINNING - 0.01,
                ],
            },
            Vertex {
                position: [
                    // middle
                    vertex.position[0] + 0.01,
                    vertex.position[1] - DEF_THINNING - 0.04,
                ],
            },
            Vertex {
                position: [
                    // tip
                    vertex.position[0] + 0.06,
                    vertex.position[1] - DEF_THINNING - 0.03,
                ],
            },
        ];

        let indices = (0..=2).collect();
        let (vertex_buffer1, index_buffer1) =
            generate_vertex_index_buffer(disp, &claw1_vertices, &indices);
        let (vertex_buffer2, index_buffer2) =
            generate_vertex_index_buffer(disp, &claw2_vertices, &indices);
        let program1 = generate_program(r, g, b, disp);
        let program2 = generate_program(r, g, b, disp);
        let tip1 = claw1_vertices[0];
        let tip2 = claw2_vertices[0];
        let claw1 = Claw {
            vertices: claw1_vertices,
            tip: tip1,
            vertex_buffer: vertex_buffer1,
            index_buffer: index_buffer1,
            program: program1,
        };

        let claw2 = Claw {
            vertices: claw2_vertices,
            tip: tip2,
            vertex_buffer: vertex_buffer2,
            index_buffer: index_buffer2,
            program: program2,
        };
        (Box::new(claw1), Box::new(claw2))
    }

    pub fn generate_chain(
        center_x: f32,
        center_y: f32,
        r: &str,
        g: &str,
        b: &str,
        disp: &glium::Display<WindowSurface>,
    ) -> Chain {
        let (vertices, indices, tip) = generate_vertices(center_x, center_y);

        let (vertex_buffer, index_buffer) = generate_vertex_index_buffer(disp, &vertices, &indices);
        let program = generate_program(r, g, b, disp);
        Chain {
            vertices,
            indices,
            tip,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    fn generate_vertices(center_x: f32, center_y: f32) -> (Vec<Vertex>, Vec<u32>, Vertex) {
        let mut vertices = vec![
            Vertex {
                position: [center_x + DEF_RADIUS - DEF_THINNING, center_y + DEF_HEIGHT], // top right
            },
            Vertex {
                position: [center_x - DEF_RADIUS, center_y], // bottom left
            },
            Vertex {
                position: [center_x - DEF_RADIUS + DEF_THINNING, center_y + DEF_HEIGHT], // top left
            },
        ];
        let mut indices: Vec<u32> = (0..=vertices.len() as u32).collect();

        // generate vertices and indices for circle
        let circle_segments = 100;
        for i in 3..=circle_segments {
            let theta = 2.0 * PI * (i as f32) / (circle_segments as f32);
            let x = DEF_RADIUS * theta.cos();
            let y = DEF_RADIUS * theta.sin();
            vertices.push(Vertex {
                position: [x + center_x, y + center_y],
            });
            indices.push(i as u32);
        }

        let tip = Vertex {
            position: [center_x, center_y + DEF_HEIGHT],
        };

        (vertices, indices, tip)
    }

    fn generate_vertex_index_buffer(
        disp: &glium::Display<WindowSurface>,
        vertexbuf: &Vec<Vertex>,
        indexbuf: &Vec<u32>,
    ) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u32>) {
        (
            glium::VertexBuffer::new(disp, vertexbuf).unwrap(),
            glium::IndexBuffer::new(disp, glium::index::PrimitiveType::TriangleFan, indexbuf)
                .unwrap(),
        )
    }

    fn generate_program(
        r: &str,
        g: &str,
        b: &str,
        disp: &glium::Display<WindowSurface>,
    ) -> glium::program::Program {
        let vertex_shader_src = r#"
        #version 330 core

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

        let fragment_shader_src = format!(
            r#"
        #version 330 core

        out vec4 color;

        void main() {{
            color = vec4({}, {}, {}, 1.0);
        }}
    "#,
            r, g, b
        );

        glium::Program::from_source(disp, vertex_shader_src, &fragment_shader_src, None).unwrap()
    }

    pub fn rotate(
        angle: f32,
        part: &mut dyn Part,
        disp: &glium::Display<WindowSurface>,
        center_x: f32,
        center_y: f32,
    ) -> glium::VertexBuffer<Vertex> {
        let rotation_angle = angle.to_radians();
        let rotation_matrix = [
            [rotation_angle.cos(), -rotation_angle.sin()],
            [rotation_angle.sin(), rotation_angle.cos()],
        ];
        for vertex in &mut *part.get_vertices() {
            let x = vertex.position[0] - center_x;
            let y = vertex.position[1] - center_y;

            vertex.position[0] = rotation_matrix[0][0] * x + rotation_matrix[0][1] * y + center_x;
            vertex.position[1] = rotation_matrix[1][0] * x + rotation_matrix[1][1] * y + center_y;
        }

        // modify tip of the chain
        let tip = part.get_tip();
        match tip {
            Some(vertex) => {
                let x = vertex.position[0] - center_x;
                let y = vertex.position[1] - center_y;
                vertex.position[0] =
                    rotation_matrix[0][0] * x + rotation_matrix[0][1] * y + center_x;
                vertex.position[1] =
                    rotation_matrix[1][0] * x + rotation_matrix[1][1] * y + center_y;
            }
            None => {}
        }

        glium::VertexBuffer::new(disp, &part.get_vertices()).unwrap()
    }

    pub fn rotate_all(
        angle: f32,
        parts: &mut HashMap<&str, Box<dyn Part>>,
        disp: &glium::Display<WindowSurface>,
        center_x: f32,
        center_y: f32,
    ) {
        for (_, part) in parts.iter_mut() {
            let vertexbuf = rotate(angle, part.as_mut(), disp, center_x, center_y);
            part.set_vertex_buf(vertexbuf);
        }
    }

    pub fn base(
        lr1: (&mut i32, &mut i32),
        lr2: (&mut i32, &mut i32),
        lr3: (&mut i32, &mut i32),
        lr4: (&mut i32, &mut i32),
        _base: &mut (i32, i32, i32, i32),
        r1: (f32, f32),
        r2: (f32, f32),
        r3: (f32, f32),
        r4: (f32, f32),
        r5: (f32, f32),
        parts: &mut HashMap<&str, Box<dyn Part>>,
        disp: &glium::Display<WindowSurface>,
    ) {
        match *_base {
            (1, 0, 0, 0) => {
                if *lr1.0 != 0 {
                    rotate_all(3.0, parts, disp, r1.0, r1.1);

                    *lr1.0 -= 1;
                    *lr1.1 += 1;
                } else {
                    *_base = (0, 1, 0, 0);
                }
                thread::sleep(SLEEP_DURATION);
            }
            (0, 1, 0, 0) => {
                if *lr2.0 != 0 {
                    let chain1 = parts.remove("chain1").unwrap();
                    rotate_all(3.0, parts, disp, r2.0, r2.1);
                    parts.insert("chain1", chain1);

                    *lr2.0 -= 1;
                    *lr2.1 += 1;
                } else {
                    *_base = (0, 0, 1, 0);
                }
                thread::sleep(SLEEP_DURATION);
            }
            (0, 0, 1, 0) => {
                if *lr3.0 > 30 {
                    let chain1 = parts.remove("chain1").unwrap();
                    let chain2 = parts.remove("chain2").unwrap();
                    rotate_all(3.0, parts, disp, r3.0, r3.1);
                    parts.insert("chain1", chain1);
                    parts.insert("chain2", chain2);

                    *lr3.0 -= 1;
                    *lr3.1 += 1;
                } else if *lr3.0 < 30 {
                    let chain1 = parts.remove("chain1").unwrap();
                    let chain2 = parts.remove("chain2").unwrap();
                    rotate_all(-3.0, parts, disp, r3.0, r3.1);
                    parts.insert("chain1", chain1);
                    parts.insert("chain2", chain2);

                    *lr3.0 += 1;
                    *lr3.1 -= 1;
                } else {
                    *_base = (0, 0, 0, 1);
                }
                thread::sleep(SLEEP_DURATION);
            }
            (0, 0, 0, 1) => {
                if *lr4.0 != 0 {
                    let claw1_vb = rotate(
                        5.0,
                        parts.get_mut("claw1").unwrap().as_mut(),
                        disp,
                        r4.0,
                        r4.1,
                    );
                    let claw2_vb = rotate(
                        -5.0,
                        parts.get_mut("claw2").unwrap().as_mut(),
                        disp,
                        r5.0,
                        r5.1,
                    );
                    parts.get_mut("claw1").unwrap().set_vertex_buf(claw1_vb);
                    parts.get_mut("claw2").unwrap().set_vertex_buf(claw2_vb);

                    *lr4.0 -= 1;
                    *lr4.1 += 1;
                } else {
                    *_base = (0, 0, 0, 0);
                }
                thread::sleep(SLEEP_DURATION);
            }
            _ => {}
        }
    }

    pub fn execute(
        lr1: (&mut i32, &mut i32),
        lr2: (&mut i32, &mut i32),
        lr3: (&mut i32, &mut i32),
        lr4: (&mut i32, &mut i32),
        _state: &mut (i32, i32, i32, i32),
        r1: (f32, f32),
        r2: (f32, f32),
        r3: (f32, f32),
        r4: (f32, f32),
        r5: (f32, f32),
        parts: &mut HashMap<&str, Box<dyn Part>>,
        disp: &glium::Display<WindowSurface>,
        state: &mut State,
    ) {
        match *_state {
            (1, 0, 0, 0) => {
                if *lr1.0 > state.l1 {
                    rotate_all(3.0, parts, disp, r1.0, r1.1);

                    *lr1.0 -= 1;
                    *lr1.1 += 1;
                } else if *lr1.0 < state.l1 {
                    rotate_all(-3.0, parts, disp, r1.0, r1.1);

                    *lr1.0 += 1;
                    *lr1.1 -= 1;
                } else {
                    *_state = (0, 1, 0, 0);
                }
                thread::sleep(SLEEP_DURATION);
            }
            (0, 1, 0, 0) => {
                if *lr2.0 > state.l2 {
                    let chain1 = parts.remove("chain1").unwrap();
                    rotate_all(3.0, parts, disp, r2.0, r2.1);
                    parts.insert("chain1", chain1);

                    *lr2.0 -= 1;
                    *lr2.1 += 1;
                } else if *lr2.0 < state.l2 {
                    let chain1 = parts.remove("chain1").unwrap();
                    rotate_all(-3.0, parts, disp, r2.0, r2.1);
                    parts.insert("chain1", chain1);

                    *lr2.0 += 1;
                    *lr2.1 -= 1;
                } else {
                    *_state = (0, 0, 1, 0);
                }
                thread::sleep(SLEEP_DURATION);
            }
            (0, 0, 1, 0) => {
                if *lr3.0 > state.l3 {
                    let chain1 = parts.remove("chain1").unwrap();
                    let chain2 = parts.remove("chain2").unwrap();
                    rotate_all(3.0, parts, disp, r3.0, r3.1);
                    parts.insert("chain1", chain1);
                    parts.insert("chain2", chain2);

                    *lr3.0 -= 1;
                    *lr3.1 += 1;
                } else if *lr3.0 < state.l3 {
                    let chain1 = parts.remove("chain1").unwrap();
                    let chain2 = parts.remove("chain2").unwrap();
                    rotate_all(-3.0, parts, disp, r3.0, r3.1);
                    parts.insert("chain1", chain1);
                    parts.insert("chain2", chain2);

                    *lr3.0 += 1;
                    *lr3.1 -= 1;
                } else {
                    *_state = (0, 0, 0, 1);
                }
                thread::sleep(SLEEP_DURATION);
            }
            (0, 0, 0, 1) => {
                if *lr4.0 > state.l4 {
                    let claw1_vb = rotate(
                        5.0,
                        parts.get_mut("claw1").unwrap().as_mut(),
                        disp,
                        r4.0,
                        r4.1,
                    );
                    let claw2_vb = rotate(
                        -5.0,
                        parts.get_mut("claw2").unwrap().as_mut(),
                        disp,
                        r5.0,
                        r5.1,
                    );
                    parts.get_mut("claw1").unwrap().set_vertex_buf(claw1_vb);
                    parts.get_mut("claw2").unwrap().set_vertex_buf(claw2_vb);

                    *lr4.0 -= 1;
                    *lr4.1 += 1;
                } else if *lr4.0 < state.l4 {
                    let claw1_vb = rotate(
                        -5.0,
                        parts.get_mut("claw1").unwrap().as_mut(),
                        disp,
                        r4.0,
                        r4.1,
                    );
                    let claw2_vb = rotate(
                        5.0,
                        parts.get_mut("claw2").unwrap().as_mut(),
                        disp,
                        r5.0,
                        r5.1,
                    );
                    parts.get_mut("claw1").unwrap().set_vertex_buf(claw1_vb);
                    parts.get_mut("claw2").unwrap().set_vertex_buf(claw2_vb);

                    *lr4.0 += 1;
                    *lr4.1 -= 1;
                } else {
                    *_state = (0, 0, 0, 0);
                }
                thread::sleep(SLEEP_DURATION);
            }
            _ => {}
        }
    }

    pub fn apply_gravity(
        part: &mut dyn Part,
        disp: &glium::Display<WindowSurface>,
    ) -> glium::VertexBuffer<Vertex> {
        for vertex in &mut *part.get_vertices() {
            vertex.position[1] -= 0.01;
        }
        glium::VertexBuffer::new(disp, &part.get_vertices()).unwrap()
    }

    fn check_boundaries(ray_start: Vertex, first: Vertex, second: Vertex) -> bool {
        let (x, y) = (ray_start.position[0], ray_start.position[1]);
        let (x1, y1) = (first.position[0], first.position[1]);
        let (x2, y2) = (second.position[0], second.position[1]);

        if ((y1 <= y && y < y2) || (y2 <= y && y < y1)) && x < f32::max(x1, x2) {
            return true;
        }
        false
    }

    fn ray_edge_intersect(ray_start: Vertex, first: Vertex, second: Vertex) -> f32 {
        let (x, y) = (ray_start.position[0], ray_start.position[1]);
        let (x1, y1) = (first.position[0], first.position[1]);
        let (x2, y2) = (second.position[0], second.position[1]);

        let scaling_factor = ((x1 - x) * (y2 - y1) - (y1 - y) * (x2 - x1)) / (y2 - y1);

        x + scaling_factor + 0.07
    }

    pub fn detect_collision(claw1: &dyn Part, claw2: &dyn Part, vertices: &Vec<Vertex>) -> bool {
        let mut res = false;
        let length = vertices.len();

        // number of intersections
        let (mut num_intersec_right, mut num_intersec_left) = (0, 0);
        for i in 0..length {
            // get all the edges in a cyclic manner
            let first = vertices[i];
            let second = vertices[(i + 1) % length];

            if check_boundaries(claw1.get_vertices_ref()[2], first, second)
                && check_boundaries(claw2.get_vertices_ref()[2], first, second)
            {
                let intersec_right = ray_edge_intersect(claw1.get_vertices_ref()[2], first, second);
                let intersec_left = ray_edge_intersect(claw2.get_vertices_ref()[2], first, second);

                if claw1.get_vertices_ref()[2].position[0] < intersec_right {
                    num_intersec_right += 1;
                }

                if claw2.get_vertices_ref()[2].position[0] < intersec_left {
                    num_intersec_left += 1;
                }
            }
        }

        if num_intersec_left % 2 == 1 && num_intersec_right % 2 == 1 {
            res = true;
        }

        res
    }

    pub fn create(
        display: &glium::Display<WindowSurface>,
    ) -> (HashMap<&str, Box<dyn Part>>, Object) {
        let mut chain1: Box<dyn Part> =
            Box::new(generate_chain(-0.5, -0.4, "1.0", "0.6", "0.0", display));
        let mut chain2: Box<dyn Part> = Box::new(generate_chain(
            chain1.get_tip().unwrap().position[0],
            chain1.get_tip().unwrap().position[1],
            "0.0",
            "1.0",
            "0.0",
            display,
        ));
        let mut chain3: Box<dyn Part> = Box::new(generate_chain(
            chain2.get_tip().unwrap().position[0],
            chain2.get_tip().unwrap().position[1],
            "0.0",
            "0.0",
            "1.0",
            display,
        ));
        let chain3_buf = rotate(
            -90.0,
            chain3.as_mut(),
            display,
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

        let obj = generate_object(vertices, "0.0", "0.0", "0.0", display);

        let (claw1, claw2) =
            generate_claws(*chain3.get_tip().unwrap(), "1.0", "0.0", "0.0", display);
        let mut parts: HashMap<&str, Box<dyn Part>> = HashMap::new();
        parts.insert("chain1", chain1);
        parts.insert("chain2", chain2);
        parts.insert("chain3", chain3);
        parts.insert("claw1", claw1);
        parts.insert("claw2", claw2);
        (parts, obj)
    }
}
