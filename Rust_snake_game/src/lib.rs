
pub type Position = (usize , usize);
#[derive(Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right
}
#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    // head is the first item and tail is the last item
    snake: Vec<Position>,
    direction : Direction,
    food: Position, 
}

impl SnakeGame {
    pub fn new(width: usize, height: usize)-> Self {
        Self {
            width,
            height,
            snake: vec![((width-2).max(0),height/2)],
            direction: Direction::Left,
            food:(2.min(width-1), height/2)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test(){
        println!("{:?}", SnakeGame::new(10, 10));
    }
}