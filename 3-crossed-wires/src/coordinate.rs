use crate::movement::{Movement, Movement::*}; 

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    
    pub fn origin() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0
        }
    }
    
    pub fn clone_with_move(&self, m: Movement) -> Coordinate {        
        match m {
            Left(d) => Coordinate {
                x: self.x - d,
                ..*self
            },
            Right(d) => Coordinate {
                x: self.x + d,
                ..*self
            },
            Up(d) => Coordinate {
                y: self.y + d,
                ..*self
            },
            Down(d) => Coordinate {
                y: self.y - d,
                ..*self
            }
        }
    }
    
    pub fn distance_to(&self, coordinate: &Coordinate) -> i32 {
        (coordinate.x - self.x).abs() + (coordinate.y - self.y).abs() 
    }
}