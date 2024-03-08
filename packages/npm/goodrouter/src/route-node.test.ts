import { permutations } from "itertools";
import assert from "node:assert/strict";
import test from "node:test";
import { RouteNode } from "./route-node.js";
import { defaultRouterOptions } from "./router-options.js";
import { parseTemplatePairs } from "./template.js";

test("route-node-permutations", () => {
  const routeConfigs = ["/a", "/b/{x}", "/b/{x}/", "/b/{x}/c", "/b/{x}/d", "/b/e/{x}/f"];

  const permutedRouteConfigs = permutations(routeConfigs, routeConfigs.length);

  let rootNodePrevious: RouteNode<string> | null = null;

  for (const routeConfigs of permutedRouteConfigs) {
    const rootNode = new RouteNode<string>();

    for (const template of routeConfigs) {
      const templatePairs = [
        ...parseTemplatePairs(template, defaultRouterOptions.parameterPlaceholderRE),
      ];
      rootNode.insert(template, templatePairs);
    }

    {
      assert.equal(rootNode.countChildren(), 1);
    }

    if (rootNodePrevious != null) {
      assert.deepEqual(rootNode, rootNodePrevious);
    }

    rootNodePrevious = rootNode;
  }
});

test("route-node-sort", () => {
  const nodes: RouteNode<string>[] = [
    new RouteNode("aa"),
    new RouteNode("xx"),
    new RouteNode("aa", true),
    new RouteNode("x"),
  ];

  const nodesExpected = [...nodes];
  const nodesActual = [...nodes].sort((a, b) => a.compare(b));

  assert.deepEqual(nodesActual, nodesExpected);
});
