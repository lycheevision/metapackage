use std::ops::Add;
use super::{Point, Size};

#[derive(Debug)]
pub struct Rectangle<I = f64> {
    /// The position of the top left corner of the rectangle in relation to a reference origin.
    pub origin: Point<I>,
    /// The dimensions.
    pub dimensions: Size<I>,
}

impl<I> Rectangle<I> where I: Copy {

    pub fn min_x(&self) -> I {
        self.origin.x
    }
    
    pub fn min_y(&self) -> I {
        self.origin.y
    }
    
    pub fn min(&self) -> Point<I> {
        Point { x: self.min_x(), y: self.min_y() }
    }
    
    pub fn max_x(&self) -> I where I: Add<Output=I> {
        self.origin.x + self.dimensions.width
    }
    
    pub fn max_y(&self) -> I where I: Add<Output=I> {
        self.origin.y + self.dimensions.height
    }
    
    /// Returns the maximum point within the bounds of the rectangle.
    pub fn max(&self) -> Point<I> where I: Add<Output=I> {
        Point { x: self.max_x(), y: self.max_y() }  
    }
    
    /// Returns the point at the top-right corner.
    pub fn top_right(&self) -> Point<I> where I: Add<Output=I> {
        Point { x: self.max_x(), y: self.min_y() }
    }
    
    /// Returns the point at the bottom-left corner.
    pub fn bottom_left(&self) -> Point<I> where I: Add<Output=I> {
        Point { x: self.min_x(), y: self.max_y() }
    }
    
    /// Returns the point at the bottom-right corner.
    pub fn bottom_right(&self) -> Point<I> where I: Add<Output=I> {
        self.max()
    }
}

impl<I> Clone for Rectangle<I> where I: Clone {
    fn clone(&self) -> Rectangle<I> {
        Rectangle {
            origin: self.origin.clone(),
            dimensions: self.dimensions.clone()
        }
    }
}

impl<I> Copy for Rectangle<I> where I: Copy { }

impl<I> From<(I, I, I, I)> for Rectangle<I> {
    fn from((x, y, width, height): (I, I, I, I)) -> Rectangle<I> {
        Rectangle {
            origin: Point { x, y},
            dimensions: Size { width, height }
        }
    }
}

impl<I> From<[I; 4]> for Rectangle<I> {
    fn from([x, y, width, height]: [I; 4]) -> Rectangle<I> {
        Rectangle {
            origin: Point { x, y},
            dimensions: Size { width, height }
        }
    }
}

impl<I> Into<(I, I, I, I)> for Rectangle<I> {
    fn into(self) -> (I, I, I, I) {
        (self.origin.x, self.origin.y, self.dimensions.width, self.dimensions.height)
    }
}

impl<I> Into<[I; 4]> for Rectangle<I> {
    fn into(self) -> [I; 4] {
        [self.origin.x, self.origin.y, self.dimensions.width, self.dimensions.height]
    }
}

impl<I> PartialEq for Rectangle<I> where I: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.dimensions == other.dimensions
    }
}

impl<I> Eq for Rectangle<I> where I: Eq { }