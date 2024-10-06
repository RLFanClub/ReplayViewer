use std::sync::Arc;

use winit::window::Window;


pub struct Description {
    window: Arc<Window>,
    background: wgpu::Color,
    surface: wgpu::Surface<'static>
}

pub struct Object {
    desc: Description,
    config: wgpu::SurfaceConfiguration
}

impl Description {
    pub fn new(window: Arc<Window>, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = instance.create_surface(window.clone()).expect("Failed to create surface");
        Self {
            window,
            background,
            surface
        }
    }
    pub fn build(self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Object {
        let size = self.window.inner_size();
        let config = self.surface.get_default_config(adapter, size.width, size.height).unwrap();
        self.surface.configure(device, &config);
        Object { desc: self, config }
    }

    pub fn surface(&self) -> &wgpu::Surface<'static> {
        &self.surface
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn background(&self) -> &wgpu::Color {
        &self.background
    }
}

impl Object {
    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.desc.surface.configure(device, &self.config);
    }

    pub fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc.surface.get_current_texture().expect("Unable to acquire swap chain texture")
    }

    pub fn description(&self) -> &Description {
        &self.desc
    }
}
