use std::cell::UnsafeCell;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Node {
    id: i32,
    point: UnsafeCell<Point>,
}

impl Node {
    pub fn new() -> Node {
        Node {id: 0, point: UnsafeCell::new(Point {x: 0, y: 0})}
    }

    pub fn set_id(&mut self, number: i32) {
        self.id = number;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn set_point(&mut self, x: i32, y: i32) {
        self.point = UnsafeCell::new(Point {x: x, y: y});
    }

    pub fn get_point(&mut self) -> *mut Point {
        self.point.get()
    }
}
