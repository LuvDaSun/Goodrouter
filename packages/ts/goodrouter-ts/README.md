# Goodrouter, TypeScript edition

Check out our [website](https://www.goodrouter.org), the [documentation](https://ts.goodrouter.org). And feel free to join our [Discord server](https://discord.gg/BJ8v7xTq8d)!

## Example

```typescript
const router = new Router();

router.insertRoute("all-products", "/product/all");
router.insertRoute("product-detail", "/product/{id}");

// And now we can parse routes!

{
  const [routeName] = router.parseRoute("/not-found");
  assert.equal(routeName, null);
}

{
  const [routeName] = router.parseRoute("/product/all");
  assert.equal(routeName, "all-products");
}

{
  const [routeName, routeParameters] = router.parseRoute("/product/1");
  assert.equal(routeName, "product-detail");
  assert.deepEqual(routeParameters, { id: "1" });
}

// And we can stringify routes

{
  const path = router.stringifyRoute("all-products");
  assert.equal(path, "/product/all");
}

{
  const path = router.stringifyRoute("product-detail", { id: "2" });
  assert.equal(path, "/product/2");
}
```
