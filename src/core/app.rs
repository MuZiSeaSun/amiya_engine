use sdl2::{Sdl, event::Event};
use std::{thread, sync::mpsc::channel, time};

use crate::core::window::Window;

pub struct App{

    sdl: Sdl,
    window: Window,
    
    logic_ctx: LogicContex
}

struct LogicContex{
    is_quit: bool,
    is_pause: bool,
    last_tic: u32, 
    now_tic: u32
}

impl LogicContex {
    fn new() -> LogicContex{
        LogicContex { is_quit: false, is_pause: false, last_tic: 0, now_tic: 0}
    }
}

impl Clone for LogicContex {
    fn clone(&self) -> Self {
        Self { is_quit: self.is_quit, is_pause: self.is_pause, last_tic: self.last_tic, now_tic: self.now_tic}
    }
}

impl App {
    pub fn new() -> App{
        let (sdl,  window) = App::init_sdl();

        App { 
            sdl,
            window,
            logic_ctx: LogicContex::new() 
        }
    }

    pub fn run(&mut self){
        let start = time::Instant::now();
        while !self.logic_ctx.is_quit {
            let now = time::Instant::now();
            let during: u32 = now.duration_since(start).as_millis().try_into().expect("during to big");

            let (logic_ctx_sender, logic_ctx_receiver) = channel();
            if during - self.logic_ctx.now_tic > 10 {
                let mut contex = self.logic_ctx.clone();
                contex.now_tic = contex.now_tic + 20;
                thread::spawn(move||{
                    App::fix_update(&mut contex);
                    logic_ctx_sender.send(contex).expect("send err");
                });
            }



            let mut event_pump = self.sdl.event_pump().map_err(|e| e.to_string()).expect("msg");
            for event in event_pump.poll_iter(){
                self.event_handle(&event);
            }
        }
    }

    fn event_handle(&mut self, event: &Event){
        match event {
            Event::Quit{..} => { self.logic_ctx.is_quit; },
            _ => {print!("hhh");}
        }
    }

    fn fix_update(ctx: &mut LogicContex){
            
    }

    fn init_sdl() -> (Sdl, Window){
        let sdl_context = sdl2::init().expect("msg");
        let window = Window::new(&sdl_context);

        return (sdl_context, window);
    }
}