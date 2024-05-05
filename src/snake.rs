//pub struct Position {
//    // coordinates should be normalized
//    // because it usually depends on screen size
//    // as in 0-1 or -0.5 to 0.5
//
//    // let's assume for now, that it's not
//    // normalized
//    x: f16,
//    y: f16,
//}
//
//impl Position {
//    fn origin() -> Position {
//        Position { x: 0.0, y: 0.0 }
//    }
//
//    // does it take normalized params already
//    // or any set of coords and returns normalized?
//    fn new(x: f16, y: f16) -> Position {
//        // to normalize, I need a screen size
//        // -- width and height, so I can divide the
//        // -- values by it
//        Position { x, y }
//    }
//}

pub struct Snake {
    size: u8, //goes from 0-255. Consider u16 if 255 isn't enough
    //position: Position,
}

impl Snake {
    pub fn new() -> Snake {
        Snake { size: 3 }
    }
    pub fn eat(&mut self) {
        self.size += 1;
        println!("omnomonomonom my size is now {}", self.size);
    }

    //fn move(&self, x:f16, y: f16) -> void {
    //    // &mut self ?? 
    //    self.position.x += x;
    //    self.position.y += y;
    //    println!("moving to new position")
    //}
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}
