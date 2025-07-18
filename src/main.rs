extern crate sdl2;
mod textures;
mod behaviour;
mod game;
mod Colors;
use Colors::colors::sdl;
use game::run;

const GAMENAME:&'static str = "Rogue"; 
const WIDTH:u32 = 800; 
const HEIGHT:u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(GAMENAME , WIDTH, HEIGHT).position_centered().build().unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(sdl::GREY);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();
    run(canvas, event_pump,texture_creator);

}
