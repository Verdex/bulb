
pub struct Screen {
    pub width : usize,
    pub height : usize,
}

impl Screen {
    pub fn new(width : usize, height : usize) -> Screen {
        Screen { width, height }
    }
}