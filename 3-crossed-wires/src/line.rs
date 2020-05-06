use crate::coordinate::Coordinate; 

#[derive(Debug, PartialEq, Clone)]
pub enum LineOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub p1: Coordinate, 
    pub p2: Coordinate,
    pub orientation: LineOrientation,
}

impl Line {
    
    pub fn new(p1: Coordinate, p2: Coordinate) -> Self {
        use LineOrientation::*;
        
        let mut orientation = Horizontal;
        
        if p1.x == p2.x {
            orientation = Vertical;
        }
        
        Line {
            p1: p1,
            p2: p2,
            orientation: orientation
        }
    }
    
    fn left_bound(&self) -> &Coordinate {
        if self.p1.x < self.p2.x {
            return &self.p1
        } else {
            return &self.p2
        }
    }
    
    fn right_bound(&self) -> &Coordinate {
        if self.p1.x > self.p2.x {
            return &self.p1
        } else {
            return &self.p2
        }
    }
    
    fn lower_bound(&self) -> &Coordinate {
        if self.p1.y < self.p2.y {
            return &self.p1
        } else {
            return &self.p2
        }
    }
    
    fn upper_bound(&self) -> &Coordinate {
        if self.p1.y > self.p2.y {
            return &self.p1
        } else {
            return &self.p2
        }
    }
    
    fn sort_by_orientation<'a>(line_1: &'a Line, line_2: &'a Line) -> (&'a Line, &'a Line) {
        if line_1.orientation == LineOrientation::Horizontal {
            return (line_1, line_2)
        } else {
            return (line_2, line_1)
        }
    }
    
    pub fn intersection(&self, line: &Line) -> Option<Coordinate> {
        if self.orientation == line.orientation {
            return None
        }
        
        let (horizontal_line, vertical_line) = Line::sort_by_orientation(self, line);
        
        let is_horizontal_overlap = horizontal_line.left_bound().x <= vertical_line.p1.x && horizontal_line.right_bound().x >= vertical_line.p1.x;
        if !is_horizontal_overlap {
            return None
        }
        
        let is_vertical_overlap = vertical_line.upper_bound().y >= horizontal_line.p1.y && vertical_line.lower_bound().y <= horizontal_line.p1.y;
        if !is_vertical_overlap {
            return None
        }

        Some(Coordinate {
            x: vertical_line.p1.x,
            y: horizontal_line.p1.y
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_overlap() {
        let horizontal_line = Line::new(Coordinate {
            x: -2,
            y: -2
        }, Coordinate {
            x: -10,
            y: -2
        });

        let vertical_line = Line::new(Coordinate {
            x: -5,
            y: -2
        }, Coordinate {
            x: -5,
            y: 10
        });

        if let Some(intersection) = horizontal_line.intersection(&vertical_line) {
            assert_eq!(intersection.x, -5);
            assert_eq!(intersection.y, -2);
        } else {
            panic!("Expected lines to intersect!!");
        }
    }
}