use std::ops::{Add, Sub};

#[derive (Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

#[derive (Debug, Clone)]
pub struct Rectangle<T> {
    pub bottom_left: Point<T>,
    pub top_right: Point<T>
}

pub trait HasMinMax<T> {
    const MIN: T;
    const MAX: T;
}

impl<T: HasMinMax<T>> Rectangle<T> {
    pub fn whole() -> Self {
        let bottom_left = Point { x: T::MIN, y: T::MIN };
        let top_right = Point { x: T::MAX, y: T::MAX };
        Rectangle { bottom_left, top_right }
    }
}

impl<T: Ord> Rectangle<T> {
    pub fn is_empty(&self) -> bool {
        self.bottom_left.x > self.top_right.x ||
        self.bottom_left.y > self.top_right.y
    }

    pub fn interects(&self, other: &Rectangle<T>) -> bool {
        !self.is_empty() && !other.is_empty() &&
        self.bottom_left.x <= other.top_right.x &&
        self.top_right.x >= other.bottom_left.x &&
        self.bottom_left.y <= other.top_right.y &&
        self.top_right.y >= other.bottom_left.y     
    }

}

#[derive (Debug, Clone)]
pub struct RectangleSet<T>(Vec<Rectangle<T>>);

impl<T> RectangleSet<T> {
    pub fn new() -> Self {
        RectangleSet(Vec::new())
    }

    pub fn iter_rectangles(&self) -> std::slice::Iter<'_, Rectangle<T>> {
        self.0.iter()
    }
}

impl<T: Ord> RectangleSet<T> {
    pub fn from_rectangle(rectangle: Rectangle<T>) -> Self {
        if rectangle.is_empty() {
            Self::new()
        }
        else {
            RectangleSet(vec![rectangle])
        }
    }
}

impl<T: HasMinMax<T> + Ord> RectangleSet<T> {
    pub fn whole() -> Self {
        Self::from_rectangle(Rectangle::whole())
    }
}

impl<T: Ord + Clone + Sub<Output=T> + Add<Output=T> + From<u8>> RectangleSet<T> {
    pub fn diff(&self, other: &Rectangle<T>) -> Self {
        let mut result = Vec::new ();

        for rect in &self.0 {
            if !rect.interects(other) {
                result.push(rect.clone()); // Keep the whole rectangle
                continue;
            }

            // Left rectangle
            if rect.bottom_left.x < other.bottom_left.x {
                result.push(Rectangle {
                    bottom_left: rect.bottom_left.clone(),
                    top_right: Point {
                        x: other.bottom_left.x.clone() - T::from(1),
                        y: rect.top_right.y.clone()
                    }
                });
            }

            // Right rectangle
            if rect.top_right.x > other.top_right.x {
                result.push(Rectangle {
                    bottom_left: Point {
                        x: other.top_right.x.clone() + T::from(1),
                        y: rect.bottom_left.y.clone()
                    },
                    top_right: rect.top_right.clone()
                });
            }

            // Middle rectangles
            let left = rect.bottom_left.x.clone().max(other.bottom_left.x.clone());
            let right = rect.top_right.x.clone().min(other.top_right.x.clone());

            // Bottom Middle rectangle
            if rect.bottom_left.y < other.bottom_left.y {
                result.push(Rectangle {
                    bottom_left: Point {
                        x: left.clone(),
                        y: rect.bottom_left.y.clone()
                    },
                    top_right: Point {
                        x: right.clone(),
                        y: other.bottom_left.x.clone() - T::from(1)
                    }
                });
            }

            // Top Right rectangle
            if rect.top_right.y > other.top_right.y {
                result.push(Rectangle {
                    bottom_left: Point {
                        x: left,
                        y: other.top_right.y.clone() + T::from(1)
                    },
                    top_right: Point {
                        x: right,
                        y: rect.top_right.y.clone()
                    }
                });
            }
        }

        RectangleSet(result)
    }
}
