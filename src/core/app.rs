use sdl2::{Sdl, event::Event};

use crate::core::window::Window;

pub struct App{
    is_quit: bool,
    is_pause: bool,
    last_fixtic: u32,
    last_drawtick: u32,

    sdl: Sdl,
    window: Window
}

impl App {
    pub fn new() -> App{
        let (sdl,  window) = App::init_sdl();
        App { 
            is_quit: false, 
            is_pause: false, 
            last_fixtic: 0, 
            last_drawtick: 0, 
            sdl,
            window,
        }
    }

    pub fn run(&mut self){
        let mut event_pump = self.sdl.event_pump().map_err(|e| e.to_string()).expect("msg");
        
        while !self.is_quit {
            for event in event_pump.poll_iter(){
                self.event_handle(&event);
            }
        }
    }

    fn event_handle(&mut self, event: &Event){
        match event {
            Event::Quit{..} => { self.is_quit = true; },
            _ => {print!("hhh");}
        }
    }

    fn init_sdl() -> (Sdl, Window){
        let sdl_context = sdl2::init().expect("msg");
        let window = Window::new(&sdl_context);

        return (sdl_context, window);
    }
}