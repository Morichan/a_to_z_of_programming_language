pub struct Node {
    pub id: i32,
}

impl Node {
    pub fn new() -> Node {
        Node {id: 0}
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, number: i32) {
        self.id = number;
    }
}
