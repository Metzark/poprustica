use std::{
    collections::HashMap,
    sync::Arc
};
use wgpu;
use winit::window::Window;

use super::internal::{self, Sprite};

pub struct Grafx {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    shader_map: HashMap<u8, wgpu::ShaderModule>,
    bind_group_map: HashMap<String, wgpu::BindGroup>,
    sprite_map: HashMap<String, internal::Sprite>
}

impl Grafx {
    pub async fn new(window: Arc<Window>) -> Self {
        // The physical size of the winit window
        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();

        // Context for all wgpu objects
        let instance: wgpu::Instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        // The surface is what is everything is drawn to
        let surface: wgpu::Surface<'static> = instance.create_surface(window.clone()).unwrap();

        // The adapter is for actaully handing GPU
        let adapter: wgpu::Adapter = internal::create_adapter(&instance, &surface).await;

        // The GPU device and device queue
        let (device, queue): (wgpu::Device, wgpu::Queue) = internal::create_device_and_queue(&adapter).await;

        // The surface configuration
        let config: wgpu::SurfaceConfiguration = internal::create_surface_configuration(&adapter, &surface, &size);

        // Configure surface for presentation
        surface.configure(&device, &config);

        // Create a single bind group layout to be shared by all bind groups
        let bind_group_layout: wgpu::BindGroupLayout = internal::create_bind_group_layout(&device);

        let shader: wgpu::ShaderModule = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/sprite.wgsl").into()),
        });

        // Create a render pipeline layout to be used by render pipelines
        let render_pipeline_layout: wgpu::PipelineLayout = internal::create_render_pipeline_layout(&device, &bind_group_layout);

        // Create a render pipe (going to use one for everything for now)
        let render_pipeline: wgpu::RenderPipeline = internal::create_render_pipeline(&device, &render_pipeline_layout, &shader, &config);

        // Create map of shaders
        let shader_map: HashMap<u8, wgpu::ShaderModule> = HashMap::new();

        // Create map of bind groups
        let bind_group_map: HashMap<String, wgpu::BindGroup> = HashMap::new();

        let mut sprite_map: HashMap<String, internal::Sprite> = HashMap::new();

        let diffuse_bytes = include_bytes!("../assets/bg1.png");
        let diffuse_texture = internal::Texture::from_bytes(&device, &queue, diffuse_bytes, "../assets/bg1.png").unwrap();

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        // bind_group_map.insert(String::from("background"), diffuse_bind_group);

        let background: internal::Sprite = internal::Sprite::fullscreen_quad(&device, &diffuse_bind_group);

        sprite_map.insert(String::from("background"), background);
     
        let grafx = Grafx {
            window,
            device,
            queue,
            surface,
            size,
            config,
            render_pipeline,
            shader_map,
            bind_group_map,
            sprite_map
        };

        grafx
    }

    /// Render a single frame
    pub fn render(&mut self) -> Result<(), anyhow::Error> {
        let output = self.surface.get_current_texture().unwrap();

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        let mut rendered = false;

        match self.sprite_map.get("background") {
            Some(sprite) => {
                sprite.draw(&mut render_pass);
                rendered = true;
            }
            None => {
                println!("No render");
            }
        }

        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        if rendered {
            return Ok(());
        }

        return Err(anyhow::anyhow!("Sprite 'background' not found"));

    }

    /// Get the game window
    pub fn get_window(&self) -> &Window {
        &self.window
    }
}