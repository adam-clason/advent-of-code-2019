#[derive(Debug)]
pub enum Movement {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

impl Movement {
    
    pub fn parse(from_str: &str) -> Movement {
        use Movement::*;
    
        let (direction, distance) = from_str.split_at(1);
        let distance: i32 = distance.parse().unwrap();
        
        match direction {
            "L" => Left(distance),
            "R" => Right(distance),
            "U" => Up(distance),
            "D" => Down(distance),
            _ => panic!("Unexpected Movement Value {}", from_str),
        }
    }
}
