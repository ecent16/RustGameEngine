/*
    Description: Entry point for accessing pipeline and vertex settings for the engine render.
    Main file for renderering,
*/
use std::sync::Arc;
use wgpu::*;
use winit::window::Window;

// Include all files to be shared
pub mod pipeline; // Export pipeline so other crates can use it
pub mod vertex;

use pipeline::PipelineConfig;

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration, 
    pub win_size: winit::dpi::PhysicalSize<u32>,
    pipeline: PipelineConfig, // Reference Pipeline file for return type
}

impl Renderer {

    pub async fn new(window: Arc<Window>) -> Self{

        let win_size = window.inner_size();

        // Part 1: Instance - entry point to wgpu, sets native backend
        let instance = wgpu::Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY, 
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap(); // Tie wgpu to winit window

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance, 
                compatible_surface: Some(&surface), 
                force_fallback_adapter: false, 
            }
        )
        .await
        .expect("no suitable GPU adapter found"); // Handle Physical GPU

        let(device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"), 
                    required_features: wgpu::Features::empty(), 
                    required_limits: wgpu::Limits::default(), 
                }, 
            None, 
            )
            .await
            .expect("failed to create device"); // Logical device and its command submission queue

        // Part 2 Surface Configurations
        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats
            .iter()
            .find(|i| i.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
            format: surface_format, 
            width: win_size.width, 
            height: win_size.height, 
            present_mode: wgpu::PresentMode::Fifo, 
            alpha_mode: surface_caps.alpha_modes[0], 
            view_formats: vec![], 
            desired_maximum_frame_latency: 2, 
        };

        surface.configure(&device, &surface_config);

        let pipeline = PipelineConfig::new_render_pipeline(&device, surface_format); 

        Self{ device, queue, surface, surface_config, win_size, pipeline }
    }

    pub fn resize_window(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {

        if new_size.width == 0 || new_size.height == 0 { return; }

        self.win_size = new_size;
        self.surface_config.height = new_size.height; 
        self.surface_config.width =  new_size.width;
        self .surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> { // Error Check

        let output = self.surface.get_current_texture()?; // Get next texture from the swapchain
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: Some("Render Encoder")}
        );

        // Render Pass - clears the screen and issues draw cells
        {
            // Set the encoder values in this scope
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view, 
                    resolve_target: None, 
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, g: 0.1, b: 0.1, a: 1.0, 
                        }), 
                        store: wgpu::StoreOp::Store, 
                    }
                })], 
                depth_stencil_attachment: None, 
                ..Default::default()
            });

            pass.set_pipeline(self.pipeline.render_pipeline()); // Get the render pipeline
            pass.draw(0..3, 0..1);
        }

        // Submit commands to the GPU queue
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

