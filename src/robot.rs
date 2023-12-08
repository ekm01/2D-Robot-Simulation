pub mod robot {
    use glium::glutin::surface::WindowSurface;
    use std::f32::consts::PI;

    #[derive(Copy, Clone, Debug)]
    pub struct Vertex {
        pub position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    const DEF_RADIUS: f32 = 0.05;
    pub const DEF_THINNING: f32 = 0.02;
    pub const DEF_HEIGHT: f32 = 0.4;
    pub const GROUND: f32 = -0.45;

    pub trait Part {
        fn get_vertex_buf(&self) -> &glium::VertexBuffer<Vertex>;
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32>;
        fn get_program(&self) -> &glium::program::Program;
        fn get_vertices(&mut self) -> &mut Vec<Vertex>;
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
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32> {
            &self.index_buffer
        }
        fn get_program(&self) -> &glium::program::Program {
            &self.program
        }
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            &mut self.vertices
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
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32> {
            &self.index_buffer
        }
        fn get_program(&self) -> &glium::program::Program {
            &self.program
        }
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            &mut self.vertices
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
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32> {
            &self.index_buffer
        }
        fn get_program(&self) -> &glium::program::Program {
            &self.program
        }
        fn get_vertices(&mut self) -> &mut Vec<Vertex> {
            &mut self.vertices
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
    ) -> (Claw, Claw) {
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
        (claw1, claw2)
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

    pub fn gravity(
        part: &mut dyn Part,
        disp: &glium::Display<WindowSurface>,
    ) -> glium::VertexBuffer<Vertex> {
        for vertex in &mut *part.get_vertices() {
            vertex.position[1] -= 0.01;
        }
        glium::VertexBuffer::new(disp, &part.get_vertices()).unwrap()
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

    pub fn detect_collision(claw1: &Claw, claw2: &Claw, vertices: &Vec<Vertex>) -> bool {
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
}
