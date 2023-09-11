use sdl2::{Sdl, surface::Surface, pixels::PixelFormatEnum, render::Canvas};

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

    pub fn draw(&mut self, vec_date: Vec<u8>, width: u32, height: u32, pitch: u32){
        let mut vec_date = vec_date;
        let date = &mut vec_date[..];
        let surface = Surface::from_data(date, width, height, pitch, PixelFormatEnum::RGBA8888).expect("");
        let texture_creator = self.canvas.texture_creator();
        let tex = surface.as_texture(&texture_creator).expect("err");
        self.canvas.copy(&tex, Option::None, Option::None).expect("err");
        self.canvas.present();
    }
}