#![feature(plugin)]

#[macro_use]
extern crate glium;
#[plugin]
extern crate glium_macros;
extern crate glium_text;
extern crate glutin;
extern crate nalgebra;
extern crate ui;

use std::default::Default;
use std::sync::Arc;
use nalgebra::Mat4;
use glium::Surface;

pub struct UiSystem {
    text: glium_text::TextSystem,
    default_font: Arc<glium_text::FontTexture>,
    rectangle: (glium::vertex::VertexBufferAny, glium::IndexBuffer),
    rectangles_program: glium::Program,
    images_program: glium::Program,
}

impl UiSystem {
    pub fn new(display: &glium::Display) -> UiSystem {
        UiSystem {
            text: glium_text::TextSystem::new(display),

            default_font: Arc::new({
                let file = std::io::fs::File::open(&Path::new("C:\\Windows\\Fonts\\Arial.ttf"));
                glium_text::FontTexture::new(display, file, 70).ok().unwrap()   // FIXME: remove ok()
            }),

            rectangle: (
                {
                    #[vertex_format]
                    #[derive(Copy)]
                    struct Vertex {
                        position: [f32; 2],
                        tex_coords: [f32; 2],
                    }

                    glium::VertexBuffer::new(display, 
                        vec![
                            Vertex { position: [0.0, 0.0], tex_coords: [0.0, 0.0] },
                            Vertex { position: [0.0, 2.0], tex_coords: [0.0, 1.0] },
                            Vertex { position: [2.0, 2.0], tex_coords: [1.0, 1.0] },
                            Vertex { position: [2.0, 0.0], tex_coords: [1.0, 0.0] }
                        ]
                    ).into_vertex_buffer_any()
                },

                glium::IndexBuffer::new(display,
                                        glium::index_buffer::TriangleStrip(vec![1 as u16, 2, 0, 3]))
            ),

            rectangles_program: glium::Program::from_source(display, r"
                #version 110

                uniform mat4 matrix;

                attribute vec2 position;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                }
            ", r"
                #version 110
                uniform vec3 color;

                void main() {
                    gl_FragColor = vec4(color, 1.0);
                }
            ", None).unwrap(),

            images_program: glium::Program::from_source(display, r"
                #version 110

                uniform mat4 matrix;

                attribute vec2 position;
                attribute vec2 tex_coords;

                varying vec2 v_tex_coords;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ", r"
                #version 110
                uniform sampler2D texture;
                varying vec2 v_tex_coords;

                void main() {
                    gl_FragColor = texture2D(texture, v_tex_coords);
                }
            ", None).unwrap()
        }
    }

    pub fn draw<T, U>(&self, target: &mut T, ui: &ui::Ui<U>) where T: Surface, U: ui::Component {
        for shape in ui.draw().iter() {
            match shape {
                &ui::Shape::Point { .. } => unimplemented!(),
                &ui::Shape::Line { .. } => unimplemented!(),

                &ui::Shape::Rectangle { ref from, ref to, ref color } => {
                    let uniforms = uniform! {
                        matrix: [
                            [(to.x - from.x) / 2.0, 0.0, 0.0, 0.0],
                            [0.0, (to.y - from.y) / 2.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [from.x, from.y, 0.0, 1.0f32]
                        ],
                        color: color.clone()
                    };

                    target.draw(&self.rectangle.0, &self.rectangle.1, &self.rectangles_program,
                                &uniforms, &Default::default());
                },

                &ui::Shape::Text { ref text, ref font, ref bottom_left, ref em } => {
                    let em = *em * 20.0;        // FIXME: why?
                    let text = glium_text::TextDisplay::new(&self.text, self.default_font.clone(),
                                                            &text[]);

                    let mat = Mat4::new(em, 0.0, 0.0, bottom_left.x,       // TODO: perspective
                                           0.0, em, 0.0, bottom_left.y,
                                           0.0,   0.0, 1.0, 0.0,
                                           0.0,   0.0, 0.0, 1.0);

                    glium_text::draw(&text, &self.text, target, mat, (1.0, 1.0, 1.0, 1.0));
                },

                _ => {}
            }
        }
    }
}
