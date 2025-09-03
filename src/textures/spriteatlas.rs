use std::path::PathBuf;
use sdl2::render::{TextureCreator,Texture,WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use std::cmp::Ordering;
use crate::textures::screen::Screen;
use crate::behaviour::object::object;

const atlas_path:&'static str = "/home/argument/rust/Rogue-rust/Assets/sprites/atlas.png";
pub const PIXELSIZE:u32 = 32;
pub const GLOBALSCALE:u32 = 3;

pub const SPRITES:[sprite;1] = [sprite{name:"brick_tile",offset:{offset{R1:(128,48,64,64),R2:(0,0,PIXELSIZE*GLOBALSCALE,PIXELSIZE*GLOBALSCALE)}},scale:1,multiple:true,cuttype:Spritecut::ByPixelSize}];

#[derive(Clone)]
pub struct offset{
    pub R1:(i32,i32,u32,u32),
    pub R2:(i32,i32,u32,u32),
}
 
#[derive(Clone)]
pub struct sprite{
    offset:offset,
    scale:i32,
    name:&'static str,
    multiple:bool,
    cuttype:Spritecut,
}

#[derive(Clone)]
enum Spritecut{
    Null,
    ByPixelSize,
}




pub fn load_atlas(texture_creator:&TextureCreator<WindowContext>) -> Texture{
      let atlas = texture_creator.load_texture(atlas_path);

      match atlas{
            Ok(a) => return a,
            Err(..) => panic!("not a file"),
      }
}


pub fn draw_sprite(screen:&mut Screen,obj:&object){
    let sprite = &obj.Sprite;
    let Rec1 = Rect::new(sprite.offset.R1.0,sprite.offset.R1.1,sprite.offset.R1.2,sprite.offset.R1.3);
    let Rec2 = Rect::new(obj.Position.x as i32,obj.Position.y as i32,sprite.offset.R2.2,sprite.offset.R2.3);

    screen.canvas.copy(&screen.atlas,Rec1,Rec2).unwrap();
}

pub fn getsprite(sp_name:&str) -> Vec<&sprite>{
    let mut current_sprite:Option<&sprite> = None;
    let mut sprites:Vec<&sprite> = Vec::new();
    for i in &SPRITES[0..SPRITES.len()]{

        if i.name.contains(sp_name){
            current_sprite = i.into();
        }
    }

    
    match current_sprite{
        None => panic!("not a sprite"),
        Some(sp) => {
                let current_sprite = sp;
                    sprites.push(current_sprite);
                

                sprites
            }

        }
}


