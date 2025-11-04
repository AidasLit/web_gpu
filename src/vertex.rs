use std::collections::HashMap;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

enum Colours {
    Red,
    Green,
    Blue,
    DarkOrange,
    DarkRose,
    White,
}

const COLOUR_CODES : &[[f32; 3]; 6] = &[
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.835, 0.176, 0.0],
    [0.639, 0.007, 0.384],
    [1.0, 1.0, 1.0],
];

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [0.5, 0.0, 0.0], color: COLOUR_CODES[Colours::White as usize] }, // D
    Vertex { position: [-0.5, -0.5, 0.0], color: COLOUR_CODES[Colours::DarkRose as usize] }, // A
    Vertex { position: [0.5, -0.5, 0.0], color: COLOUR_CODES[Colours::DarkRose as usize] }, // B

    Vertex { position: [0.5, 0.0, 0.0], color: COLOUR_CODES[Colours::White as usize] }, // D
    Vertex { position: [-0.5, 0.0, 0.0], color: COLOUR_CODES[Colours::White as usize] }, // C
    Vertex { position: [-0.5, -0.5, 0.0], color: COLOUR_CODES[Colours::DarkRose as usize] }, // A

    Vertex { position: [0.0, 0.5, 0.0], color: COLOUR_CODES[Colours::DarkOrange as usize] }, // E
    Vertex { position: [-0.5, 0.0, 0.0], color: COLOUR_CODES[Colours::White as usize] }, // C
    Vertex { position: [0.5, 0.0, 0.0], color: COLOUR_CODES[Colours::White as usize] }, // D
];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}