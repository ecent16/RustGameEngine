/*
    Description: Pipeline file for rendering.
*/
pub fn create_render_pipeline(
    device: &wgpu::Device, 
    surface_format: wgpu::TextureFormat, 
) -> wgpu::RenderPipeline {

    let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/triangle.wgsl"));
    
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"), 
        bind_group_layouts: &[], 
        push_constant_ranges: &[], 
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Main Pipeline"), 
        layout: Some(&layout), 

        vertex: wgpu::VertexState {
            module: &shader, 
            entry_point: "vs_main", 
            buffers: &[],
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
            strip_index_format: None, 
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: Some(wgpu::Face::Back), 
            ..Default::default()
        }, 

        depth_stencil: None, 
        multisample: wgpu::MultisampleState::default(), 
        multiview: None,
    })

}