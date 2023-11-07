# Goodrouter, the rust edition

Check out our [website](https://www.goodrouter.org), join our [Discord server](https://discord.gg/BJ8v7xTq8d)!

## Example

```rust
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
```
