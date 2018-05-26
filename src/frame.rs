
use glium;
use glium::{
    Surface,
    Program,

    DrawParameters,
    index::{
        PrimitiveType
    }
};


use trap::Matrix4;


use color::Color;
use draw::{
    Draw,
    Triangles
};

use vertex::Vertex;
use vertex_array::VertexArray;
use projection::Projection;
use projection::View;

pub struct Frame<'a> {
    frame: glium::Frame,

    vertex_array: &'a mut VertexArray,

    program: &'a Program,
    draw_parameters: DrawParameters<'a>,

    projection_matrix: Matrix4,
    view_matrix: Matrix4,
}


impl<'a> Frame<'a> {
    pub fn new(mut frame: glium::Frame, vertex_array: &'a mut VertexArray, program: &'a Program) -> Frame<'a> {
        use glium::*;

        vertex_array.clear_all_vertices();
        frame.clear_all((1.0, 1.0, 1.0, 1.0), 1.0, 1);

        self::Frame {
            frame,
            vertex_array,

            program,
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },



                .. Default::default()
            },

            projection_matrix: Matrix4::new(),
            view_matrix: Matrix4::new(),
        }
    }


    pub fn clear(&mut self, color: Color) {
        self.flush();

        self.frame.clear_color(
            color.r as f32,
            color.g as f32,
            color.b as f32,
            color.a as f32
        );
    }


    /// Render something
    pub fn draw(&mut self, object: &Draw) {
        match object.triangulate() {
            Triangles::IndexedList { vertices, indices } => {
                self.vertex_array.append_vertices(&vertices, &indices, PrimitiveType::TrianglesList);
            },
        }
    }



    /// Set the projection mode
    pub fn set_projection(&mut self, projection: Projection) {
        self.flush();

        match projection {
            Projection::Perspective { fov, aspect, near, far } => {
                self.projection_matrix = Matrix4::perspective(fov, aspect, near, far);
            },

            Projection::Orthographic {left, right, top, bottom, near, far } => {
                self.projection_matrix = Matrix4::orthographic(left, right, top, bottom, near, far);
            }
        }
    }


    /// Set the view mode
    pub fn set_view(&mut self, view: View) {
        self.flush();

        match view {
            View::LookAt { eye, target, up } => {
                self.view_matrix = Matrix4::look_at(eye, target, up);
            },

            View::None => {
                self.view_matrix = Matrix4::new();
            }
        }
    }


    /// Flush the vertex array
    pub fn flush(&mut self) {
        for (vertex_buffer, index_buffer) in self.vertex_array.get_vertices() {
            let uniforms = uniform!(
                projection: mat_to_arr(self.projection_matrix),
                view: mat_to_arr(self.view_matrix)
            );

            self.frame.draw(
                vertex_buffer,
                index_buffer,
                &self.program,
                &uniforms,
                &self.draw_parameters
            ).unwrap();
        }

        self.vertex_array.clear_all_vertices();
    }
}


impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        self.flush();

        // Finish drawing on drop
        self.frame.set_finish().unwrap()
    }
}

fn mat_to_arr(matrix: Matrix4) -> [[f32; 4]; 4] {
    matrix.into()
}