use crate::textures::spriteatlas::{sprite,getsprite,draw_sprite,SPRITES,PIXELSIZE,GLOBALSCALE};
use crate::textures::screen::Screen;

const ACTOBJBEFORERUN:[(&'static str,Pos,u32);1] = [("brick_tile",Pos{x:0.0,y:0.0},8)];


#[derive(Clone)]
pub struct Pos{
    pub x:f32,
    pub y:f32,
}

#[derive(Clone)]
pub struct object{
    pub Sprite:sprite,
    pub Layer:i32,
    pub Position:Pos,
    pub Enabled:bool,
}



impl object{
    
    pub fn create_object(screen:&mut Screen,sprite:&sprite,layer:i32,position:Pos){
        let obj = object{ Sprite:sprite.clone(), Layer:layer, Position:position,Enabled:true};
        draw_sprite(screen,&obj); 
        screen.allobj.push(obj);
    }

    pub fn create_existing_objects(screen:&mut Screen){
        
        for obj_descr in &ACTOBJBEFORERUN{
            let sprite = getsprite(obj_descr.0);
            let mut sprite_index = 0;

            for tiles_times in 0..obj_descr.2{
                if sprite_index == sprite.len()
                {
                    sprite_index = 0;
                }

                let layer = 0;
                let position = Pos{y:obj_descr.1.y,x:obj_descr.1.x + (PIXELSIZE*GLOBALSCALE*tiles_times) as f32};          
                object::create_object(screen,sprite[sprite_index],layer,position);
                sprite_index += 1;
            }
        }
    }
}
