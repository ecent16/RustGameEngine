/*
    Description: Pipeline file for processing and rendering. You can optimize gpu settings here.
    This script uses wgpu, and vertex crate structures.

    Pipeline Layout: Define shader program
    Render Pipeline: Configure graphics for rendering tasks
    Descriptor Sets: Group resources for efficient binding to shader programs.

    Notes: 
    - Research on how to make the struct private

*/
use crate::vertex::Vertex;
pub struct PipelineConfig {
    render_pipeline: wgpu::RenderPipeline, // wgpu struct specific type
    // Additional pipelines go here...
}

impl PipelineConfig {

    pub fn new_render_pipeline(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {

        let shader = device.create_shader_module(
            // Shader module settings
            wgpu::ShaderModuleDescriptor {
                label: Some("Pipeline Shaders"), 
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/triangle.wgsl").into()),  
            }
        );

        // Define the vertex buffer layout
        let vertex_buffer_layout = Vertex::buffer_layout(); // Initialize Vertex struct
        
        let layout = device.create_pipeline_layout(
            // Pipeline layout settings
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"), 
                bind_group_layouts: &[], 
                push_constant_ranges: &[], 
            }
        );

        let render_pipeline = device.create_render_pipeline(
            // Adjust render settings here
            &wgpu::RenderPipelineDescriptor {
                label: Some("Main Pipeline"), 
                layout: Some(&layout), 

                vertex: wgpu::VertexState {
                    module: &shader, 
                    entry_point: "vs_main", 
                    buffers: &[vertex_buffer_layout], // Reference vertex buffer, 
                }, 

                fragment: Some(wgpu::FragmentState {
                    module: &shader, 
                    entry_point: "fs_main",  
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_format, 
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                        write_mask: wgpu::ColorWrites::ALL, 
                    })],
                }), 

                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw, 
                    cull_mode: Some(wgpu::Face::Back), 
                    ..Default::default()
                }, 

                depth_stencil: None, 
                multisample: wgpu::MultisampleState::default(), 
                multiview: None, 
            }
        );

        Self { render_pipeline } 
    }

    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

}