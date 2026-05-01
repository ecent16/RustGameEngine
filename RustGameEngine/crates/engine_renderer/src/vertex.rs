/*
    Description: Vertex structure for the engine pipeline
*/
use wgpu::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]

pub struct Vertex {
    pub position: [f32; 3], 
    pub uv: [f32; 2], 
    pub normal: [f32; 3], 
}

impl Vertex {

    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0, 
                    shader_location: 0, 
                    format: wgpu::VertexFormat::Float32x3, // Position
                }, 
                // Optional Attribute here
                wgpu::VertexAttribute {
                    offset: 12, 
                    shader_location: 1, 
                    format: wgpu::VertexFormat::Float32x2, // UV
                }, 

                wgpu::VertexAttribute {
                    offset: 20, 
                    shader_location: 2, 
                    format: wgpu::VertexFormat::Float32x2, // Normal
                }
            ], 
        }
    }
}
