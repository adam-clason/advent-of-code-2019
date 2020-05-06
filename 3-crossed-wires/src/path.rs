use std::collections::LinkedList;

use crate::coordinate::Coordinate;
use crate::movement::Movement;
use crate::line::Line;

#[derive(Debug, Clone)]
struct PathLine {
    line: Line,
    steps_to: i32,
}


impl PathLine {

    fn origin() -> Self {
        PathLine {
            line: Line::new(Coordinate::origin(), Coordinate::origin()),
            steps_to: 0,
        }
    }

    fn new_with_move(last: PathLine, movement: Movement) -> Self {
        let last_line  = last.line;
        let steps_to = last.steps_to + (last_line.p2.x - last_line.p1.x).abs() + 
            (last_line.p2.y - last_line.p1.y).abs();
        let next_p2 = last_line.p2.clone_with_move(movement);
        let line = Line::new(last_line.p2, next_p2);

        PathLine {
            line: line,
            steps_to: steps_to,
        } 
    }
}


#[derive(Debug)]
pub struct Path {
    path_lines: LinkedList<PathLine>,
}

impl Path {
    
    pub fn generate(moves_str: Vec<String>) -> Path {
        let mut movements = moves_str
            .iter()
            .map(|s| Movement::parse(s));

        let mut lines = LinkedList::new(); 
        
        let mut prev_line = PathLine::origin();
        
        while let Some(movement) = movements.next() {
            let current_line = PathLine::new_with_move(prev_line, movement);
            
            lines.push_back(current_line.clone());
            
            prev_line = current_line;
        }
        
        Path {
            path_lines: lines
        }
    }

    pub fn closest_intersect(&self, path: &Path) -> Option<Coordinate> {
        let origin = Coordinate::origin();
        self.path_lines.iter().
            flat_map(|self_line| {
                path.path_lines.iter().filter_map(move |other_line| self_line.line.intersection(&other_line.line))
            })
            .filter(|c| c.x != 0 || c.y != 0)
            .min_by(|c1, c2| c1.distance_to(&origin).cmp(&c2.distance_to(&origin)))
    }

     pub fn shortest_walk(&self, path: &Path) -> Option<i32> {
        self.path_lines.iter()
            .flat_map(|self_line| {
                path.path_lines.iter().filter_map(move |other_line| {
                    if let Some(intersection) = self_line.line.intersection(&other_line.line) {
                        let new_steps = self_line.line.p1.distance_to(&intersection) + 
                            other_line.line.p1.distance_to(&intersection);

                        return Some((
                            intersection, 
                            self_line.steps_to + 
                            other_line.steps_to + 
                            new_steps
                        ))
                    } else {
                        return None
                    }
                })
            })
            .filter(|(c, _)| c.x != 0 || c.y != 0)
            .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
            .map(|t| t.1)        
     }
    
}
