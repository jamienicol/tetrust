extern crate sdl2;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use sdl2::render::{ Renderer, RenderDriverIndex, ACCELERATED, PRESENTVSYNC };
use sdl2::video::{ Window, WindowPos, OPENGL };

fn main() {
    let sdl = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let window = match Window::new(
        &sdl, "Tetrust",
        WindowPos::PosCentered, WindowPos::PosCentered, 480, 960,
        OPENGL)
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let mut renderer = match Renderer::from_window(
        window, RenderDriverIndex::Auto, ACCELERATED | PRESENTVSYNC)
    {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut drawer = renderer.drawer();

    let mut running = true;
    while running {
        for event in sdl.event_pump().poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: KeyCode::Escape, .. } => {
                    running = false
                },
                _ => {}
            }
        }

        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
        drawer.present();
    }
}
