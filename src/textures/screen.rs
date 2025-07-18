use sdl2::video::WindowContext;
use sdl2::render::{Texture,WindowCanvas};
use crate::behaviour::object::object;

pub struct Screen<'a>{
    pub canvas:WindowCanvas,
    pub atlas:Texture<'a>,    
    pub allobj:Vec<object>,
}


impl Screen<'_>{

    pub fn create_screen(canvas:WindowCanvas, atlas:Texture<'_>, allobj:Vec<object>) ->Screen{
        Screen{ canvas:canvas,atlas:atlas,allobj:allobj} 
    }
}
