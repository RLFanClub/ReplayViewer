mod viewport;

use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window
};

async fn run(event_loop: EventLoop<()>, window: &Arc<Window>) {
    let instance = wgpu::Instance::default();
    let (mut viewport, adapter, device, queue) = {
        let viewport = viewport::Description::new(window.clone(), wgpu::Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }, &instance);
        let options = wgpu::RequestAdapterOptions { compatible_surface: Some(viewport.surface()), ..Default::default() };
        let adapter = instance.request_adapter(&options).await.expect("Failed to find an appropriate adapter");
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },None,
        ).await.expect("Failed to create device");
        (viewport.build(&adapter, &device), adapter, device, queue)
    };
    event_loop.run(move |event, target| {
        let _ = (&instance, &adapter);
        if let Event::WindowEvent { window_id: _, event } = event {
            match event {
                WindowEvent::Resized(new_size) => {
                    viewport.resize(&device, new_size);
                    viewport.description().window().request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    let frame = viewport.get_current_texture();
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{label: None});
                    {let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(viewport.description().background().clone()),
                                    store: wgpu::StoreOp::Store,
                                },
                            },
                        )],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });}
                    queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                WindowEvent::CloseRequested => {
                    target.exit();
                }
                _ => {}
            }
        }
    }).unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(winit::window::WindowBuilder::new().with_title("RL Replay Viewer").build(&event_loop).unwrap());
    pollster::block_on(run(event_loop, &window));
}
