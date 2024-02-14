use super::*;
use std::{cell::RefCell, rc::Rc};

pub fn route_node_merge<'r, K>(
    parent_node_rc: RouteNodeRc<'r, K>,
    child_node_rc: Option<RouteNodeRc<'r, K>>,
    anchor: &'r str,
    has_parameter: bool,
    route_key: Option<K>,
    route_parameter_names: Vec<&'r str>,
    common_prefix_length: usize,
) -> RouteNodeRc<'r, K> {
    if let Some(child_node_rc) = child_node_rc {
        let common_prefix = &anchor[..common_prefix_length];
        let child_anchor = child_node_rc.borrow().anchor;

        if child_anchor == anchor {
            return route_node_merge_join(child_node_rc, route_key, route_parameter_names.clone());
        } else if child_anchor == common_prefix {
            return route_node_merge_add_to_child(
                parent_node_rc,
                child_node_rc,
                anchor,
                has_parameter,
                route_key,
                route_parameter_names.clone(),
                common_prefix_length,
            );
        } else if anchor == common_prefix {
            return route_node_merge_add_to_new(
                parent_node_rc,
                child_node_rc,
                anchor,
                has_parameter,
                route_key,
                route_parameter_names.clone(),
                common_prefix_length,
            );
        } else {
            return route_node_merge_intermediate(
                parent_node_rc,
                child_node_rc,
                anchor,
                has_parameter,
                route_key,
                route_parameter_names.clone(),
                common_prefix_length,
            );
        }
    } else {
        return route_node_merge_new(
            parent_node_rc,
            anchor,
            has_parameter,
            route_key,
            route_parameter_names.clone(),
        );
    }
}

fn route_node_merge_new<'r, K>(
    parent_node_rc: RouteNodeRc<'r, K>,
    anchor: &'r str,
    has_parameter: bool,
    route_key: Option<K>,
    route_parameter_names: Vec<&'r str>,
) -> RouteNodeRc<'r, K> {
    let new_node = RouteNode::<K> {
        anchor,
        has_parameter,
        route_key,
        route_parameter_names,
        parent: Some(Rc::downgrade(&parent_node_rc)),
        ..Default::default()
    };

    let node_new_rc = Rc::new(RefCell::new(new_node));
    let mut parent_node = parent_node_rc.borrow_mut();
    parent_node.children.insert(node_new_rc.clone());

    node_new_rc
}

fn route_node_merge_join<'r, K>(
    child_node_rc: RouteNodeRc<'r, K>,
    route_key: Option<K>,
    route_parameter_names: Vec<&'r str>,
) -> RouteNodeRc<'r, K> {
    let mut child_node = child_node_rc.borrow_mut();

    if child_node.route_key.is_some() && route_key.is_some() {
        panic!("ambiguous route")
    }

    if child_node.route_key.is_none() {
        child_node.route_key = route_key;
        child_node.route_parameter_names = route_parameter_names;
    }

    child_node_rc.clone()
}

fn route_node_merge_intermediate<'r, K>(
    parent_node_rc: RouteNodeRc<'r, K>,
    child_node_rc: RouteNodeRc<'r, K>,
    anchor: &'r str,
    has_parameter: bool,
    route_key: Option<K>,
    route_parameter_names: Vec<&'r str>,
    common_prefix_length: usize,
) -> RouteNodeRc<'r, K> {
    let new_node = RouteNode {
        anchor,
        has_parameter,
        route_key,
        route_parameter_names,
        ..Default::default()
    };

    let new_node_rc = Rc::new(RefCell::new(new_node));

    // remove the child from parent
    {
        let mut parent_node = parent_node_rc.borrow_mut();
        parent_node.children.remove(&child_node_rc);
    }

    // create an intermediate node
    let intermediate_node_rc = {
        let child_node = child_node_rc.borrow();

        let mut intermediate_node = RouteNode {
            anchor: &child_node.anchor[..common_prefix_length],
            has_parameter: child_node.has_parameter,
            parent: Some(Rc::downgrade(&parent_node_rc)),
            ..Default::default()
        };

        intermediate_node.children.insert(child_node_rc.clone());
        intermediate_node.children.insert(new_node_rc.clone());

        // insert the intermediate node
        let mut parent_node = parent_node_rc.borrow_mut();

        let intermediate_node_rc = Rc::new(RefCell::new(intermediate_node));
        parent_node.children.insert(intermediate_node_rc.clone());

        intermediate_node_rc
    };

    // update the new and child nodes
    {
        let mut child_node = child_node_rc.borrow_mut();
        let mut new_node = new_node_rc.borrow_mut();

        new_node.parent = Some(Rc::downgrade(&intermediate_node_rc));
        new_node.anchor = &new_node.anchor[common_prefix_length..];
        new_node.has_parameter = false;

        child_node.parent = Some(Rc::downgrade(&intermediate_node_rc));
        child_node.anchor = &child_node.anchor[common_prefix_length..];
        child_node.has_parameter = false;
    }

    // return rc to the new node
    new_node_rc.clone()
}

fn route_node_merge_add_to_child<'r, K>(
    _parent_node_rc: RouteNodeRc<'r, K>,
    child_node_rc: RouteNodeRc<'r, K>,
    anchor: &'r str,
    _has_parameter: bool,
    route_key: Option<K>,
    route_parameter_names: Vec<&'r str>,
    common_prefix_length: usize,
) -> RouteNodeRc<'r, K> {
    let anchor = &anchor[common_prefix_length..];
    let has_parameter = false;

    let (common_prefix_length2, child_node_rc2) =
        route_node_find_similar_child(&child_node_rc.borrow(), anchor, has_parameter);

    return route_node_merge(
        child_node_rc.clone(),
        child_node_rc2,
        anchor,
        has_parameter,
        route_key,
        route_parameter_names,
        common_prefix_length2,
    );
}

fn route_node_merge_add_to_new<'r, K>(
    parent_node_rc: RouteNodeRc<'r, K>,
    child_node_rc: RouteNodeRc<'r, K>,
    anchor: &'r str,
    has_parameter: bool,
    route_key: Option<K>,
    route_parameter_names: Vec<&'r str>,
    common_prefix_length: usize,
) -> RouteNodeRc<'r, K> {
    let new_node = RouteNode {
        anchor,
        has_parameter,
        route_key,
        route_parameter_names,
        ..Default::default()
    };
    let new_node_rc = Rc::new(RefCell::new(new_node));

    let mut parent_node = parent_node_rc.borrow_mut();

    parent_node.children.remove(&child_node_rc);
    parent_node.children.insert(new_node_rc.clone());

    let mut new_node = new_node_rc.borrow_mut();
    new_node.children.insert(child_node_rc.clone());
    new_node.parent = Some(Rc::downgrade(&parent_node_rc));

    let mut child_node = child_node_rc.borrow_mut();
    child_node.anchor = &child_node.anchor[common_prefix_length..];
    child_node.has_parameter = false;
    child_node.parent = Some(Rc::downgrade(&new_node_rc));

    new_node_rc.clone()
}
