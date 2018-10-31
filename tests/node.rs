extern crate a_to_z_of_programming_language;

#[cfg(test)]
mod tests {
    use a_to_z_of_programming_language::a_star::node;

    #[test]
    fn checked_id_when_node_was_set_id() {
        let expected = 1;
        let mut obj = node::Node::new();

        obj.set_id(1);
        let actual = obj.get_id();

        assert_eq!(expected, actual);
    }

    #[test]
    fn checked_point_when_node_was_set_point() {
        let expected = node::Point {x: 2, y: 10};
        let mut obj = node::Node::new();

        obj.set_point(2, 10);
        let actual = obj.get_point();

        assert!((expected.x == actual.x) && (expected.y == actual.y));
    }
}

