pub mod robot {
    use glium::glutin::surface::WindowSurface;
    use std::f32::consts::PI;

    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    const DEF_RADIUS: f32 = 0.05;
    pub const DEF_THINNING: f32 = 0.02;
    pub const DEF_HEIGHT: f32 = 0.4;

    pub trait Part {
        fn get_vertex_buf(&self) -> &glium::VertexBuffer<Vertex>;
        fn get_index_buf(&self) -> &glium::IndexBuffer<u32>;
        fn get_program(&self) -> &glium::program::Program;
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
    }

    pub struct Claw {
        pub vertices: Vec<Vertex>,
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
    }

    pub fn generate_claw(
        vertices: Vec<Vertex>,
        r: &str,
        g: &str,
        b: &str,
        disp: &glium::Display<WindowSurface>,
    ) -> Claw {
        let indices = (0..=2).collect();
        let (vertex_buffer, index_buffer) = generate_vertex_index_buffer(disp, &vertices, &indices);
        let program = generate_program(r, g, b, disp);

        Claw {
            vertices,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    pub fn generate_joint(
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
        let mut indices: Vec<u32> = (0..=3).collect();

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
        part: &mut Chain,
        disp: &glium::Display<WindowSurface>,
        center_x: f32,
        center_y: f32,
    ) -> glium::VertexBuffer<Vertex> {
        let rotation_angle = angle.to_radians();
        let rotation_matrix = [
            [rotation_angle.cos(), -rotation_angle.sin()],
            [rotation_angle.sin(), rotation_angle.cos()],
        ];
        for vertex in &mut part.vertices {
            let x = vertex.position[0] - center_x;
            let y = vertex.position[1] - center_y;

            vertex.position[0] = rotation_matrix[0][0] * x + rotation_matrix[0][1] * y + center_x;
            vertex.position[1] = rotation_matrix[1][0] * x + rotation_matrix[1][1] * y + center_y;
        }

        // modify tip of the chain
        let x = part.tip.position[0] - center_x;
        let y = part.tip.position[1] - center_y;

        part.tip.position[0] = rotation_matrix[0][0] * x + rotation_matrix[0][1] * y + center_x;
        part.tip.position[1] = rotation_matrix[1][0] * x + rotation_matrix[1][1] * y + center_y;

        glium::VertexBuffer::new(disp, &part.vertices).unwrap()
    }
}
