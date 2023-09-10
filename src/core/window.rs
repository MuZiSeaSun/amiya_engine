use sdl2::{Sdl, render::Canvas};

pub struct Window{
    canvas: Canvas<sdl2::video::Window>
}

impl Window {
    pub fn new(sdl_context: &Sdl)->Window{
        let video_subsystem = sdl_context.video().expect("msg");
    
        let window = video_subsystem
            .window("rust-sdl2 demo: Window", 800, 600)
            .resizable()
            .build()
            .map_err(|e| e.to_string()).expect("msg");

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string()).expect("msg");

        Window{
            canvas
        }
    }
}