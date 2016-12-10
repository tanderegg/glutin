extern crate glutin;

use glutin::GlRequest;
use glutin::GlProfile;

mod support;

fn resize_callback(width: u32, height: u32) {
    println!("Window resized to {}x{}", width, height);
}

fn main() {
    let mut window = glutin::WindowBuilder::new()
    .with_pixel_format(24, 8)
    .with_depth_buffer(24)
    .with_stencil_buffer(8)
    .with_gl(GlRequest::Specific(glutin::Api::OpenGl, (4, 5)))
    .with_gl_profile(GlProfile::Core)
    .build().unwrap();
    window.set_title("A fantastic window!");
    window.set_window_resize_callback(Some(resize_callback as fn(u32, u32)));
    let _ = unsafe { window.make_current() };

    println!("Pixel format of the window: {:?}", window.get_pixel_format());

    let context = support::load(&window);

    for event in window.wait_events() {
        context.draw_frame((0.0, 1.0, 0.0, 1.0));
        let _ = window.swap_buffers();

        println!("{:?}", event);

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
