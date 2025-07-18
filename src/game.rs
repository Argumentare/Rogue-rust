extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::event::Event;
use std::time::Duration;
use crate::Colors::colors::sdl;
use crate::textures::spriteatlas;

pub fn run(mut canvas:WindowCanvas, mut event_pump:EventPump){

    'game: loop{    
        canvas.set_draw_color(sdl::GREY); 
        canvas.clear();

        for event in event_pump.poll_iter(){

            match event{
                Event::Quit {..} => { break 'game},

                _ => {}
            }
        }
    
        ::std::thread::sleep(Duration::new(0,1_000_000u32 /  60));
    }
}
