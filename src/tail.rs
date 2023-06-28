use bracket_terminal::prelude::Point;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tail {
    points: VecDeque<Point>,
    length: usize,
}

impl Tail {
    pub fn new(length: usize) -> Self {
        Self {
            points: VecDeque::with_capacity(length),
            length,
        }
    }

    pub fn push(&mut self, point: Point) {
        if self.points.len() == self.length {
            self.points.pop_front();
        }
        self.points.push_back(point);
    }

    pub fn inc_length(&mut self) {
        self.length += 1;
    }

    pub const fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }

    pub fn contains(&self, point: Point) -> bool {
        self.points.contains(&point)
    }
}
