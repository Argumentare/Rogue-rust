use std::path::PathBuf;
use sdl2::render::{TextureCreator,Texture,WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use std::cmp::Ordering;
const atlas_path:&'static str = "/home/argument/rust/Rogue-rust/Assets/sprites/atlas.png";
const PIXELSIZE:u32 = 32;
const GLOBALSCALE:u32 = 3;

const SPRITES:[sprite;2] = [sprite{name:"brick_tile2",offset:{offset{R1:(128,79,PIXELSIZE,PIXELSIZE,),R2:(0,0,PIXELSIZE*GLOBALSCALE,PIXELSIZE*GLOBALSCALE)}},scale:1,multiple:false},
sprite{name:"brick_tile",offset:{offset{R1:(128,79,PIXELSIZE,PIXELSIZE,),R2:(0,0,PIXELSIZE*GLOBALSCALE,PIXELSIZE*GLOBALSCALE)}},scale:1,multiple:false}];


pub struct offset{
    pub R1:(i32,i32,u32,u32),
    pub R2:(i32,i32,u32,u32),
}

pub struct sprite{
    offset:offset,
    scale:i32,
    name:&'static str,
    multiple:bool,
}


pub fn load_atlas(texture_creator:&TextureCreator<WindowContext>) -> Texture{
      let atlas = texture_creator.load_texture(atlas_path);

      match atlas{
            Ok(a) => return a,
            Err(..) => panic!("not a file"),
      }
}


pub fn draw_sprite(canvas:&mut WindowCanvas,atlas:&Texture){
    let sprite = getsprite("brick_tile");
    let Rec1 = Rect::new(sprite.offset.R1.0,sprite.offset.R1.1,sprite.offset.R1.2,sprite.offset.R1.3);
    let Rec2 = Rect::new(sprite.offset.R2.0,sprite.offset.R2.1,sprite.offset.R2.2,sprite.offset.R2.3);

    canvas.copy(atlas,Rec1,Rec2).unwrap();
}

fn getsprite(name:&str) -> &sprite{
    let mut current_sprite:Option<&sprite> = None;

    for i in &SPRITES[0..SPRITES.len()]{

        if i.name.contains(name){
            current_sprite = i.into();
        }

    }

    
    match current_sprite{
        None => panic!("not a sprite"),
        Some(sp) => {let current_sprite = sp;
                if current_sprite.multiple
                {
    
                }   
                current_sprite
            }

        }
}


