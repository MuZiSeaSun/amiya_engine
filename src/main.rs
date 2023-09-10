use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

// use amiya_engine::core::app::App;

// fn main(){
//     let mut app = App::new();
//     app.run();
// }

fn main() {
    let sdl_context = sdl2::init().expect("msg");
    let video_subsystem = sdl_context.video().expect("msg");

    let window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string()).expect("msg");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string()).expect("msg");

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string()).expect("msg");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!(
                "Window - pos({}x{}), size({}x{}): {}",
                position.0, position.1, size.0, size.1, tick
            );
            window.set_title(&title).map_err(|e| e.to_string()).expect("msg");

            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }

}
