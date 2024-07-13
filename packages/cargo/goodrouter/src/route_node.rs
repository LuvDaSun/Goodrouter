mod functions;
mod merge;
mod traits;

use std::{cell, collections::BTreeSet, rc};

#[derive(Debug)]
pub struct RouteNodeRc<'r, K>(pub rc::Rc<cell::RefCell<RouteNode<'r, K>>>);

#[derive(Debug)]
pub struct RouteNodeWeak<'r, K>(pub rc::Weak<cell::RefCell<RouteNode<'r, K>>>);

#[derive(Debug)]
pub struct RouteNode<'r, K> {
  // the route's key, if any
  pub route_key: Option<K>,
  // the route parameter names
  pub route_parameter_names: Vec<&'r str>,
  // suffix that comes after the parameter value (if any!) of the path
  anchor: &'r str,
  // does this node has a parameter
  has_parameter: bool,
  // children that represent the rest of the path that needs to be matched
  children: BTreeSet<RouteNodeRc<'r, K>>,
  // parent node, should only be null for the root node
  parent: Option<RouteNodeWeak<'r, K>>,
}
