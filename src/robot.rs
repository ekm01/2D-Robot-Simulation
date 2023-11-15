pub mod robot {
    use glium::glutin::surface::WindowSurface;
    use std::f32::consts::PI;

    #[derive(Copy, Clone)]
    pub struct Vertex {
        pub position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    const DEF_RADIUS: f32 = 0.05;
    const DEF_THINNING: f32 = 0.02;
    const DEF_HEIGHT: f32 = 0.4;

    pub fn generate_vertices(center_x: f32, center_y: f32) -> (Vec<Vertex>, Vec<u16>, Vec<f32>) {
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
        let mut indices: Vec<u16> = (0..=3).collect();

        // generate vertices and indices for circle
        let circle_segments = 100;
        for i in 3..=circle_segments {
            let theta = 2.0 * PI * (i as f32) / (circle_segments as f32);
            let x = DEF_RADIUS * theta.cos();
            let y = DEF_RADIUS * theta.sin();
            vertices.push(Vertex {
                position: [x + center_x, y + center_y],
            });
            indices.push(i as u16);
        }

        let middle = vec![center_x, center_y + DEF_HEIGHT];

        (vertices, indices, middle)
    }

    pub fn generate_program(
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
}
