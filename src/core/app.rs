use sdl2::{Sdl, rect::{Rect, Point}, event::Event, render::{Texture, Canvas}, surface::Surface, pixels::{PixelFormatEnum, Color}};
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
    now_tic: u32,
    test: u8,
    test_pos: Point,
    test_dir: i32,
}

impl LogicContex {
    fn new() -> LogicContex{
        LogicContex { 
            is_quit: false, 
            is_pause: false, 
            last_tic: 0, 
            now_tic: 0, 
            test: 0, 
            test_pos: Point::new(0, 0),
            test_dir: 1,
        }
    }
}

impl Clone for LogicContex {
    fn clone(&self) -> Self {
        Self { 
            is_quit: self.is_quit, 
            is_pause: self.is_pause, 
            last_tic: self.last_tic, 
            now_tic: self.now_tic, 
            test: self.test,
            test_pos: self.test_pos,
            test_dir: self.test_dir,
        }
    }
}

impl LogicContex {
    fn fix_update(&mut self, delta_time: u32){
        let add: u8 = (delta_time/3).try_into().expect("msg");
        if self.test > 255 - add{
            self.test = self.test - (255 - add);
        }else {
            self.test = self.test + add;
        }
        let add:i32 = delta_time.try_into().expect("msg");
        self.test_pos.x += self.test_dir * add;
        if self.test_pos.x > 300{
            self.test_dir = -1;
        }
        if self.test_pos.x < 0{
            self.test_dir = 1;
        }
    }

    fn update(&mut self, delta_time: u32){
        let add: u8 = (delta_time/3).try_into().expect("msg");
        if self.test > 255 - add{
            self.test = self.test - (255 - add);
        }else {
            self.test = self.test + add;
        }
        let add:i32 = delta_time.try_into().expect("msg");
        self.test_pos.x += self.test_dir * add;
        if self.test_pos.x > 300{
            self.test_dir = -1;
        }
        if self.test_pos.x < 0{
            self.test_dir = 1;
        }
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
        let mut debug_tic: u64 = 0;
        let mut now;
        let mut during: u32;
        let (logic_ctx_sender, logic_ctx_receiver) = channel();
        let (render_ctx_sender, render_ctx_receiver) = channel();
        let (render_sender, render_receiver) = channel();
        while !self.logic_ctx.is_quit {
            //let duration = time::Duration::from_millis(debug_tic);
            now = /*match start.checked_add(duration) {
                Some(tic)=>{tic},
                None =>{panic!("err");}
            };*/time::Instant::now();
            during = now.duration_since(start).as_millis().try_into().expect("during to big");

            let mut event_pump = self.sdl.event_pump().map_err(|e| e.to_string()).expect("msg");
            for event in event_pump.poll_iter(){
                self.event_handle(&event);
            }

            if during > self.logic_ctx.now_tic + 10 {
                self.logic_ctx.last_tic = self.logic_ctx.now_tic;
                self.logic_ctx.now_tic = self.logic_ctx.now_tic + 20;
                let mut contex = self.logic_ctx.clone();
                let logic_ctx_sender_clone = logic_ctx_sender.clone();
                let fix_thread = thread::spawn(move||{
                    contex.fix_update(contex.now_tic - contex.last_tic);
                    logic_ctx_sender_clone.send(contex).expect("send err");
                });
                fix_thread.join().expect("msg");
            }

            match logic_ctx_receiver.try_recv() {
                Ok(contex) =>{
                    self.logic_ctx = contex;
                },
                Err(_) =>{ }
            }

            if self.logic_ctx.now_tic == 0 {
                let render_ctx_sender_clone = render_ctx_sender.clone();
                let mut contex = self.logic_ctx.clone();
                contex.last_tic = contex.now_tic;
                contex.now_tic = during;
                let render_thread = thread::spawn(move||{
                    contex.update(contex.now_tic - contex.last_tic);
                    render_ctx_sender_clone.send(contex).expect("update ctx send err");
                });
                render_thread.join().expect("msg");
            }

            match render_ctx_receiver.try_recv() {
                Ok(contex) =>{
                    let render_sender_clone = render_sender.clone();
                    let update_thread = thread::spawn(move||{
                        let surface 
                            = Surface::new(720, 720, PixelFormatEnum::RGBA8888)
                            .expect("creat suferce err");
                        let mut canvas = surface.into_canvas().expect("into canvas err");
                        App::befor_render(&mut canvas);
                        App::render(&mut canvas, &contex);
                        App::adter_render(&mut canvas);
                        //let suferce = canvas.surface();
                        let surface = canvas.surface();
                        let (date, width, height, pitch) = (surface.with_lock(|date|{
                            date.to_owned()
                        }), surface.width(), surface.height(), surface.pitch());
                        render_sender_clone.send((date, width, height, pitch)).expect("render send err");
                    });
                    update_thread.join().expect("msg");

                    let render_ctx_sender_clone = render_ctx_sender.clone();
                    let mut contex = self.logic_ctx.clone();
                    if during > contex.now_tic{
                        contex.last_tic = contex.now_tic;
                    }
                    contex.now_tic = during;
                    let render_thread = thread::spawn(move||{
                        contex.update(contex.now_tic - contex.last_tic);
                        render_ctx_sender_clone.send(contex).expect("update ctx send err");
                    });
                    render_thread.join().expect("msg");
                },
                Err(_) =>{ }
            }

            match render_receiver.try_recv() {
                Ok((date, width, height, pitch)) =>{
                    self.window.draw(date, width, height, pitch);
                },
                Err(_) =>{ }
            }

            debug_tic = debug_tic + 5;
        }
    }

    fn event_handle(&mut self, event: &Event){
        match event {
            Event::Quit{..} => { self.logic_ctx.is_quit = true; },
            _ => {}//println!("hhh");
        }
    }

    fn befor_render(canvas: &mut Canvas<Surface>){
        let old_color = canvas.draw_color();
        let draw_color = Color::RGBA(255, 0, 0, 255);
        let rect = Rect::new(0, 0, 80, 80);
        canvas.set_draw_color(draw_color);
        canvas.fill_rect(rect).expect("err");
        canvas.set_draw_color(old_color);
    }

    fn render(canvas: &mut Canvas<Surface>, ctx: &LogicContex){
        let old_color = canvas.draw_color();
        let draw_color = Color::RGBA(0, ctx.test, 0, 255);
        let rect = Rect::new(ctx.test_pos.x, ctx.test_pos.x, 80, 80);
        canvas.set_draw_color(draw_color);
        canvas.fill_rect(rect).expect("err");
        canvas.set_draw_color(old_color);
    }

    fn adter_render(canvas: &mut Canvas<Surface>){
        let old_color = canvas.draw_color();
        let draw_color = Color::RGBA(0, 0, 255, 255);
        let rect = Rect::new(200, 200, 80, 80);
        canvas.set_draw_color(draw_color);
        canvas.fill_rect(rect).expect("err");
        canvas.set_draw_color(old_color);
    }

    fn init_sdl() -> (Sdl, Window){
        let sdl_context = sdl2::init().expect("msg");
        let window = Window::new(&sdl_context);

        (sdl_context, window)
    }
}