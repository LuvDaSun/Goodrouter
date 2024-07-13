pub mod route_node_merge;
pub mod route_node_rc;
pub mod route_node_utility;

use route_node_utility::*;
use std::{
  cell::RefCell,
  cmp::Ordering,
  collections::BTreeSet,
  rc::{Rc, Weak},
};

pub type RouteNodeRc<'r, K> = Rc<RefCell<RouteNode<'r, K>>>;
pub type RouteNodeWeak<'r, K> = Weak<RefCell<RouteNode<'r, K>>>;

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

impl<'r, K> Ord for RouteNode<'r, K> {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.anchor.len() < other.anchor.len() {
      return Ordering::Greater;
    }
    if self.anchor.len() > other.anchor.len() {
      return Ordering::Less;
    }

    if !self.has_parameter && other.has_parameter {
      return Ordering::Less;
    }
    if self.has_parameter && !other.has_parameter {
      return Ordering::Greater;
    }

    if self.anchor < other.anchor {
      return Ordering::Less;
    }
    if self.anchor > other.anchor {
      return Ordering::Greater;
    }

    Ordering::Equal
  }
}

impl<'r, K> PartialOrd for RouteNode<'r, K> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<'r, K> Eq for RouteNode<'r, K> {}

impl<'r, K> PartialEq for RouteNode<'r, K> {
  fn eq(&self, other: &Self) -> bool {
    self.anchor == other.anchor && self.has_parameter == other.has_parameter
  }
}

impl<'r, K> Default for RouteNode<'r, K> {
  fn default() -> Self {
    Self {
      route_key: None,
      route_parameter_names: Default::default(),
      anchor: Default::default(),
      has_parameter: Default::default(),
      children: Default::default(),
      parent: Default::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use itertools::Itertools;
  use std::iter::FromIterator;

  #[test]
  fn route_ordering() {
    let nodes = vec![
      RouteNode {
        route_key: None,
        has_parameter: false,
        anchor: "aa",
        ..Default::default()
      },
      RouteNode {
        route_key: Some(&1),
        has_parameter: false,
        anchor: "xx",
        ..Default::default()
      },
      RouteNode {
        route_key: None,
        has_parameter: true,
        anchor: "aa",
        ..Default::default()
      },
      RouteNode {
        route_key: None,
        has_parameter: false,
        anchor: "x",
        ..Default::default()
      },
    ];

    let nodes_expected = nodes.iter();
    let nodes_actual = nodes.iter().sorted();

    assert_eq!(Vec::from_iter(nodes_actual), Vec::from_iter(nodes_expected));
  }
}
