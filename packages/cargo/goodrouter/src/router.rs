use crate::{route_node::RouteNodeRc, template::TEMPLATE_PLACEHOLDER_REGEX};
use regex::Regex;
use std::hash::Hash;
use std::{borrow::Cow, collections::HashMap};

type ParameterValueEncoder = dyn Fn(&str) -> Cow<str>;
type ParameterValueDecoder = dyn Fn(&str) -> Cow<str>;

pub struct Router<'r, K> {
  root_node_rc: RouteNodeRc<'r, K>,
  leaf_nodes_rc: HashMap<K, RouteNodeRc<'r, K>>,
  maximum_parameter_value_length: usize,
  parameter_placeholder_re: &'r Regex,
  parameter_value_encoder: Box<ParameterValueEncoder>,
  parameter_value_decoder: Box<ParameterValueDecoder>,
}

impl<'r, K: Eq + Hash + Copy> Router<'r, K> {
  pub fn new() -> Self {
    fn parameter_encoder(value: &str) -> Cow<str> {
      urlencoding::encode(value)
    }
    fn parameter_decoder(value: &str) -> Cow<str> {
      urlencoding::decode(value).unwrap_or(Cow::Borrowed(value))
    }

    let parameter_value_encoder = Box::new(parameter_encoder);
    let parameter_value_decoder = Box::new(parameter_decoder);

    Self {
      root_node_rc: RouteNodeRc::default(),
      leaf_nodes_rc: HashMap::new(),
      maximum_parameter_value_length: 20,
      parameter_placeholder_re: &TEMPLATE_PLACEHOLDER_REGEX,
      parameter_value_encoder,
      parameter_value_decoder,
    }
  }

  pub fn set_maximum_parameter_value_length(&mut self, value: usize) -> &mut Self {
    self.maximum_parameter_value_length = value;

    self
  }

  pub fn set_parameter_placeholder_re(&mut self, value: &'r Regex) -> &mut Self {
    self.parameter_placeholder_re = value;

    self
  }

  pub fn set_parameter_value_encoder(&mut self, value: Box<ParameterValueEncoder>) -> &mut Self {
    self.parameter_value_encoder = value;

    self
  }

  pub fn set_parameter_value_decoder(&mut self, value: Box<ParameterValueDecoder>) -> &mut Self {
    self.parameter_value_decoder = value;

    self
  }

  pub fn insert_route(&mut self, route_key: K, template: &'r str) -> &mut Self {
    let leaf_node_rc = self
      .root_node_rc
      .insert(route_key, template, self.parameter_placeholder_re);
    self.leaf_nodes_rc.insert(route_key, leaf_node_rc);

    self
  }

  pub fn parse_route<'f>(&self, path: &'f str) -> (Option<K>, HashMap<&'r str, Cow<'f, str>>) {
    let (route_key, parameter_names, parameter_values) = self
      .root_node_rc
      .parse(path, self.maximum_parameter_value_length);

    if let Some(route_key) = route_key {
      let parameters: HashMap<_, _> = parameter_names
        .iter()
        .cloned()
        .zip(
          parameter_values
            .iter()
            .map(|parameter_value| (self.parameter_value_decoder)(parameter_value)),
        )
        .collect();

      (Some(route_key), parameters)
    } else {
      Default::default()
    }
  }

  pub fn stringify_route<'f>(
    &self,
    route_key: K,
    route_parameters: &'f HashMap<&'f str, &'f str>,
  ) -> Option<Cow<'f, str>>
  where
    'r: 'f,
  {
    if let Some(node_rc) = self.leaf_nodes_rc.get(&route_key) {
      let parameter_values: Vec<_> = node_rc
        .0
        .borrow()
        .route_parameter_names
        .iter()
        .map(|parameter_name| route_parameters.get(parameter_name).unwrap())
        .map(|parameter_value| (self.parameter_value_encoder)(parameter_value))
        .collect();

      Some(node_rc.stringify(parameter_values))
    } else {
      None
    }
  }
}

impl<'r, K: Eq + Hash + Copy> Default for Router<'r, K> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn readme() {
    let mut router = Router::new();

    router
      .insert_route("all-products", "/product/all")
      .insert_route("product-detail", "/product/{id}");

    // And now we can parse routes!

    {
      let (route_key, route_parameters) = router.parse_route("/not-found");
      assert_eq!(route_key, None);
      assert_eq!(route_parameters, Default::default());
    }

    {
      let (route_key, route_parameters) = router.parse_route("/product/all");
      assert_eq!(route_key, Some("all-products"));
      assert_eq!(route_parameters, Default::default());
    }

    {
      let (route_key, route_parameters) = router.parse_route("/product/1");
      assert_eq!(route_key, Some("product-detail"));
      assert_eq!(
        route_parameters,
        vec![("id", "1")]
          .into_iter()
          .map(|(k, v)| (k, Cow::Borrowed(v)))
          .collect()
      );
    }

    // And we can stringify routes

    {
      let route_parameters: HashMap<_, _> = vec![].into_iter().collect();
      let path = router.stringify_route("all-products", &route_parameters);
      assert_eq!(path.unwrap().into_owned(), "/product/all".to_owned());
    }

    {
      let route_parameters: HashMap<_, _> = vec![("id", "2")].into_iter().collect();
      let path = router.stringify_route("product-detail", &route_parameters);
      assert_eq!(path.unwrap().into_owned(), "/product/2".to_owned());
    }
  }

  #[test]
  fn router_1() {
    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Route {
      A,
      B,
      C,
      D,
    }

    let mut router = Router::new();
    router
      .insert_route(&Route::A, "/a")
      .insert_route(&Route::B, "/b/{x}")
      .insert_route(&Route::C, "/b/{y}/c")
      .insert_route(&Route::D, "/b/{z}/d");

    let (route_key, route_parameters) = router.parse_route("/a");
    assert_eq!(route_key.unwrap(), &Route::A);
    assert_eq!(route_parameters, vec![].into_iter().collect());

    let (route_key, route_parameters) = router.parse_route("/b/123");
    assert_eq!(route_key.unwrap(), &Route::B);
    assert_eq!(
      route_parameters,
      vec![("x", "123")]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );

    let (route_key, route_parameters) = router.parse_route("/b/456/c");
    assert_eq!(route_key.unwrap(), &Route::C);
    assert_eq!(
      route_parameters,
      vec![("y", "456")]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );

    let (route_key, route_parameters) = router.parse_route("/b/789/d");
    assert_eq!(route_key.unwrap(), &Route::D);
    assert_eq!(
      route_parameters,
      vec![("z", "789")]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );
  }

  #[test]
  fn router_2() {
    let mut router = Router::new();

    router
      .insert_route("aa", "a/{a}/a")
      .insert_route("a", "a")
      .insert_route("one", "/a")
      .insert_route("two", "/a/{x}/{y}")
      .insert_route("three", "/c/{x}")
      .insert_route("four", "/c/{y}/{z}/");

    let (route_key, _route_parameters) = router.parse_route("/a");
    assert_eq!(route_key.unwrap(), "one");

    let (route_key, route_parameters) = router.parse_route("/a/1/2");
    assert_eq!(route_key.unwrap(), "two");
    assert_eq!(
      route_parameters,
      vec![("x", "1"), ("y", "2"),]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );

    let route_key = "two";
    let route_parameters = vec![("x", "1"), ("y", "2")].into_iter().collect();
    let path = router
      .stringify_route(route_key, &route_parameters)
      .unwrap();
    assert_eq!(path, "/a/1/2");

    let (route_key, route_parameters) = router.parse_route("/c/3");
    assert_eq!(route_key.unwrap(), "three");
    assert_eq!(
      route_parameters,
      vec![("x", "3"),]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );

    let (route_key, route_parameters) = router.parse_route("/c/3/4");
    assert_eq!(route_key.unwrap(), "three");
    assert_eq!(
      route_parameters,
      vec![("x", "3/4"),]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );

    let route_key = "three";
    let route_parameters = vec![("x", "3/4")].into_iter().collect();
    let path = router
      .stringify_route(route_key, &route_parameters)
      .unwrap();
    assert_eq!(path, "/c/3%2F4");

    let (route_key, route_parameters) = router.parse_route("/c/3/4/");
    assert_eq!(route_key.unwrap(), "four");
    assert_eq!(
      route_parameters,
      vec![("y", "3"), ("z", "4"),]
        .into_iter()
        .map(|(k, v)| (k, Cow::Borrowed(v)))
        .collect(),
    );
  }

  #[test]
  fn router_templates_small() {
    router_templates("small")
  }

  #[test]
  fn router_templates_docker() {
    router_templates("docker")
  }

  #[test]
  fn router_templates_github() {
    router_templates("github")
  }

  fn router_templates(name: &str) {
    let mut path = std::path::PathBuf::new();
    path.push("..");
    path.push("..");
    path.push("..");
    path.push("fixtures");
    path.push(name);
    path.set_extension("txt");

    let templates = std::fs::read_to_string(path.as_path()).unwrap();
    let templates: Vec<_> = templates
      .split('\n')
      .map(|line| line.trim())
      .filter(|line| !line.is_empty())
      .collect();

    let mut all_parameter_names: HashSet<&str> = Default::default();

    for template in templates.iter() {
      for captures in TEMPLATE_PLACEHOLDER_REGEX.captures_iter(template) {
        all_parameter_names.insert(captures.get(1).unwrap().as_str());
      }
    }

    let all_parameter_values: Vec<_> = (0..all_parameter_names.len())
      .map(|index| format!("p{}", index))
      .collect();

    let all_parameters: HashMap<_, _> = all_parameter_names
      .into_iter()
      .zip(all_parameter_values.iter().map(|v| v.as_str()))
      .collect();

    let mut router = Router::new();
    for template in templates.iter().cloned() {
      router.insert_route(template, template);
    }

    let paths: Vec<_> = templates
      .iter()
      .map(|template| router.stringify_route(template, &all_parameters).unwrap())
      .collect();

    for index in 0..templates.len() {
      let path = &paths[index];
      let template = templates[index];

      let (route_key, route_parameters) = router.parse_route(path);

      let expected_parameters: HashMap<_, _> = route_parameters
        .keys()
        .cloned()
        .map(|key| (key, Cow::Borrowed(all_parameters[key])))
        .collect();

      assert_eq!(route_key, Some(template));
      assert_eq!(route_parameters, expected_parameters);
    }
  }
}
