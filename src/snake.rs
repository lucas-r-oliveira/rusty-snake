pub struct Position {
    // coordinates should be normalized
    // because it usually depends on screen size
    // as in 0-1 or -0.5 to 0.5

    // let's assume for now, that it's not
    // normalized
    x: f32,
    y: f32,
}

impl Position {
    fn origin() -> Position {
        Position { x: 0.0, y: 0.0 }
    }

    // does it take normalized params already
    // or any set of coords and returns normalized?
    fn new(x: f32, y: f32) -> Position {
        // to normalize, I need a screen size
        // -- width and height, so I can divide the
        // -- values by it
        Position { x, y }
    }
}

pub struct Snake {
    size: u8, //goes from 0-255. Consider u16 if 255 isn't enough
    position: Position,
}

impl Snake {
    pub fn new() -> Snake {
        Snake { size: 3, position: Position::origin() }
    }
    pub fn eat(&mut self) {
        self.size += 1;
        println!("omnomonomonom my size is now {}", self.size);
    }

    pub fn translate(&mut self, x:f32, y: f32) {
        self.position.x += x;
        self.position.y += y;
        //TODO: implement std::fmt::Display on Position (same as __repr__)
        println!("moving to new position {}, {}", self.position.x, self.position.y )
    }
}

impl Default for Snake {
    fn default() -> Self {
        Self::new()
    }
}
