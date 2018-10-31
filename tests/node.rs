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
        let expected = 1;
        let mut obj = node::Node::new();

        obj.set_point(1, 1);
        let actual = obj.get_point();

        assert_eq!(expected, unsafe{ (*actual).x });
    }
}

