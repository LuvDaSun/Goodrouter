# GoodRouter, .NET edition

Check out the [website](https://www.goodrouter.org), join our [Discord server](https://discord.gg/BJ8v7xTq8d)!

Example:

```csharp
var router = new Router();

router
    .InsertRoute("all-products", "/product/all")
    .InsertRoute("product-detail", "/product/{id}");

// And now we can parse routes!

{
    var (routeKey, routeParameters) = router.ParseRoute("/not-found");
    Assert.Null(routeKey);
    Assert.Equal(
        new Dictionary<string, string>(),
        routeParameters
    );
}

{
    var (routeKey, routeParameters) = router.ParseRoute("/product/all");
    Assert.Equal("all-products", routeKey);
    Assert.Equal(
        new Dictionary<string, string>(),
        routeParameters
    );
}

{
    var (routeKey, routeParameters) = router.ParseRoute("/product/1");
    Assert.Equal("product_detail", routeKey);
    Assert.Equal(
        new Dictionary<string, string>() {
            {"id", "1"}
        },
        routeParameters
    );
}

// And we can stringify routes

{
    var path = router.StringifyRoute("all-products");
    Assert.Equal("/product/all", path);
}

{
    var path = router.StringifyRoute(
        "product-detail",
        new Dictionary<string, string>() {
            {"id", "2"}
        }
    );
    Assert.Equal("/product/2", path);
}

```
