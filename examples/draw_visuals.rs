extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate gfx_graphics;
extern crate find_folder;
extern crate gfx;
extern crate gfx_device_gl;
use gfx::traits::*;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use gfx::memory::Typed;
use glutin_window::{GlutinWindow, OpenGL};
use piston::window::{OpenGLWindow, Window, WindowSettings};
use piston::event_loop::Events;
use graphics::*;
use piston::input::*;
use gfx_graphics::{Flip, Gfx2d, Texture, TextureSettings};

extern crate ruist;
use ruist::visual::types::*;
use ruist::draw::target_piston;

fn main() {
    let opengl = OpenGL::V3_2;
    let samples = 4;
    let mut window: GlutinWindow = WindowSettings::new("piston: draw_state", [600, 600])
        .exit_on_esc(true)
        .samples(samples)
        .opengl(opengl)
        .fullscreen(true)
        .build()
        .unwrap();

    let (mut device, mut factory) =
        gfx_device_gl::create(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

    // Create the main color/depth targets.
    let draw_size = window.draw_size();
    let aa = samples as gfx::texture::NumSamples;
    let dim = (draw_size.width as u16, draw_size.height as u16, 1, aa.into());
    let color_format = <Srgba8 as Formatted>::get_format();
    let depth_format = <DepthStencil as Formatted>::get_format();
    let (output_color, output_stencil) =
        gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0);
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);
    let mut encoder = factory.create_command_buffer().into();
    let mut g2d = Gfx2d::new(opengl, &mut factory);
    let mut events = window.events();

    let bg_color2 = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 0.5,
    };
    let bg_color1 = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    let rect_display = RectDisplay {
        x: 50.0,
        y: 50.0,
        w: 50.0,
        h: 50.0,
        backgrounds: vec![Background::Filled(bg_color1), Background::Filled(bg_color2)],
        children: Children::new(),
    };
    let test_display = VisualDisplay {
        _type: DisplayType::Rect(rect_display),
        status: Status::Enabled,
    };

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            g2d.draw(&mut encoder,
                     &output_color,
                     &output_stencil,
                     args.viewport(),
                     |c, g| {
                clear([0.8, 0.8, 0.8, 1.0], g);
                let width = c.viewport.unwrap().window_size[0] as f64;
                let height = c.viewport.unwrap().window_size[1] as f64;
                //println!("{:?}", c.viewport.unwrap().window_size);
                //println!("{:?}", c.viewport.unwrap().draw_size);

                target_piston::draw(&c, g, &test_display);
            });
            encoder.flush(&mut device);
        }

        if let Some(_) = e.after_render_args() {
            device.cleanup();
        }
    }
}
