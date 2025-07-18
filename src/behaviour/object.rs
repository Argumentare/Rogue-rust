use crate::textures::spriteatlas::sprite;

static mut ALLOBJECTS:Vec<object> = Vec::new();


pub struct Pos{
        x:i32,
        y:i32,
}

pub struct object{
    Sprite:sprite,
    Layer:i32,
    Position:Pos,
}


impl Clone for object {
    fn clone(&self) -> Self {
        *self
    }
}


impl object{

}
