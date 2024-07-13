use super::route_node_merge::*;
use super::*;
use crate::template::template_pairs::parse_template_pairs;
use regex::Regex;
use std::borrow::Cow;
use std::cmp::min;

impl<'r, K> RouteNodeRc<'r, K> {
  pub fn parse<'f>(
    &self,
    path: &'f str,
    maximum_parameter_value_length: usize,
  ) -> (Option<K>, Vec<&'r str>, Vec<&'f str>)
  where
    K: Copy,
  {
    let mut path = path;
    let mut parameter_values: Vec<&str> = Default::default();

    let node = self.0.borrow();

    if node.has_parameter {
      // we are matching a parameter value! If the path's length is 0, there is no match, because a parameter value should have at least length 1
      if path.is_empty() {
        return Default::default();
      }

      // look for the anchor in the path. If the anchor is empty, match the remainder of the path
      let index = if node.anchor.is_empty() {
        Some(path.len())
      } else {
        path[..min(
          maximum_parameter_value_length + node.anchor.len(),
          path.len(),
        )]
          .find(node.anchor)
      };

      if let Some(index) = index {
        let value = &path[..index];

        // remove the matches part from the path
        path = &path[index + node.anchor.len()..];

        parameter_values.push(value);
      } else {
        return Default::default();
      }
    } else {
      // if this node does not represent a parameter we expect the path to start with the `anchor`
      if !path.starts_with(node.anchor) {
        // this node does not match the path
        return Default::default();
      }

      // we successfully matches the node to the path, now remove the matched part from the path
      path = &path[node.anchor.len()..];
    }

    for child_rc in &node.children {
      if let (Some(child_route_name), child_route_parameter_names, mut child_parameters_values) =
        child_rc.parse(path, maximum_parameter_value_length)
      {
        let mut parameter_values = parameter_values.clone();
        parameter_values.append(&mut child_parameters_values);
        return (
          Some(child_route_name),
          child_route_parameter_names,
          parameter_values,
        );
      }
    }

    // if the node had a route name and there is no path left to match against then we found a route
    if path.is_empty() {
      if let Some(route_key) = node.route_key {
        return (
          Some(route_key),
          node.route_parameter_names.clone(),
          parameter_values,
        );
      }
    }

    Default::default()
  }

  pub fn stringify<'f>(&self, parameter_values: Vec<Cow<'f, str>>) -> Cow<'f, str>
  where
    'r: 'f,
  {
    let mut parameter_values = parameter_values.clone();
    let mut current_node_rc = Some(self.clone());
    let mut path_parts = Vec::new();

    while let Some(node_rc) = current_node_rc {
      let node = node_rc.0.borrow();
      path_parts.insert(0, Cow::Borrowed(node.anchor));

      if node.has_parameter {
        let value = parameter_values.pop().unwrap();
        path_parts.insert(0, value);
      }

      current_node_rc = node
        .parent
        .as_ref()
        .map(|parent_node_weak| parent_node_weak.try_into().unwrap());
    }

    path_parts
      .into_iter()
      .reduce(|path, path_part| path + path_part)
      .unwrap()
  }

  pub fn insert(
    &self,
    route_key: K,
    template: &'r str,
    parameter_placeholder_re: &'r Regex,
  ) -> RouteNodeRc<'r, K>
  where
    K: Copy,
  {
    let template_pairs: Vec<_> = parse_template_pairs(template, parameter_placeholder_re).collect();
    let route_parameter_names: Vec<_> = template_pairs
      .iter()
      .cloned()
      .filter_map(|(_anchor, parameter)| parameter)
      .collect();

    let mut node_current_rc = self.clone();
    for index in 0..template_pairs.len() {
      let (anchor, parameter) = template_pairs[index];
      let has_parameter = parameter.is_some();
      let route_key = if index == template_pairs.len() - 1 {
        Some(route_key)
      } else {
        None
      };

      let (common_prefix_length, child_node_rc) = node_current_rc
        .0
        .borrow()
        .find_similar_child(anchor, has_parameter);

      node_current_rc = route_node_merge(
        &node_current_rc,
        child_node_rc.as_ref(),
        anchor,
        has_parameter,
        route_key,
        route_parameter_names.clone(),
        common_prefix_length,
      );
    }

    node_current_rc
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::template::TEMPLATE_PLACEHOLDER_REGEX;
  use itertools::Itertools;

  #[test]
  fn route_node_permutations() {
    let route_configs = ["/a", "/b/{x}", "/b/{x}/", "/b/{x}/c", "/b/{y}/d"];

    let mut node_root_previous_rc = None;

    for route_configs in route_configs.iter().permutations(route_configs.len()) {
      let node_root_rc = RouteNodeRc::default();

      for template in route_configs {
        node_root_rc.insert(template, template, &TEMPLATE_PLACEHOLDER_REGEX);
      }

      {
        let node_root = node_root_rc.0.borrow();
        assert_eq!(node_root.children.len(), 1);
      }

      if let Some(node_root_previous) = node_root_previous_rc {
        assert_eq!(node_root_rc, node_root_previous);
      }

      node_root_previous_rc = Some(node_root_rc.clone());
    }
  }
}
