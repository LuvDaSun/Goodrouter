use super::*;
use crate::string_utility::find_common_prefix_length;

pub fn route_node_find_similar_child<'r, K>(
    parent_node: &RouteNode<'r, K>,
    anchor: &'r str,
    has_parameter: bool,
) -> (usize, Option<RouteNodeRc<'r, K>>) {
    let anchor_chars: Vec<_> = anchor.chars().collect();

    for child_node_rc in parent_node.children.iter() {
        if child_node_rc.borrow().has_parameter != has_parameter {
            continue;
        }

        let child_anchor_chars: Vec<_> = child_node_rc.borrow().anchor.chars().collect();

        let common_prefix_length = find_common_prefix_length(&anchor_chars, &child_anchor_chars);

        if common_prefix_length == 0 {
            continue;
        }

        return (common_prefix_length, Some(child_node_rc.clone()));
    }

    Default::default()
}
