import { RouteNodeJson } from "./route-node-json.js";
import { RouteNode } from "./route-node.js";
import { defaultRouterOptions, RouterOptions } from "./router-options.js";
import { parseTemplatePairs } from "./template.js";

/**
 * @description
 * This is the actual router that contains all routes and does the actual routing
 *
 * @example
 * ```typescript
 * const router = new Router<string>();
 *
 * router.insertRoute("all-products", "/product/all");
 * router.insertRoute("product-detail", "/product/{id}");
 *
 * // And now we can parse routes!
 *
 * {
 *   const [routeKey, routeParameters] = router.parseRoute("/not-found");
 *   assert.equal(routeKey, null);
 *   assert.deepEqual(routeParameters, {});
 * }
 *
 * {
 *   const [routeKey, routeParameters] = router.parseRoute("/product/all");
 *   assert.equal(routeKey, "all-products");
 *   assert.deepEqual(routeParameters, {});
 * }
 *
 * {
 *   const [routeKey, routeParameters] = router.parseRoute("/product/1");
 *   assert.equal(routeKey, "product-detail");
 *   assert.deepEqual(routeParameters, { id: "1" });
 * }
 *
 * // And we can stringify routes
 *
 * {
 *   const path = router.stringifyRoute(
 *     "all-products",
 *   });
 *   assert.equal(path, "/product/all");
 * }
 *
 * {
 *   const path = router.stringifyRoute(
 *     "product-detail",
 *     { id: "2" },
 *   );
 *   assert.equal(path, "/product/2");
 * }
 * ```
 */
export class Router<K extends string | number> {
    constructor(options: RouterOptions = {}) {
        this.options = {
            ...defaultRouterOptions,
            ...options,
        };
    }

    protected options: RouterOptions & typeof defaultRouterOptions;

    private rootNode = new RouteNode<K>();
    private leafNodes = new Map<K, RouteNode<K>>();
    private templatePairs = new Map<
        K,
        Array<readonly [string, string | null]>
    >();

    /**
     * @description
     * Adds a new route
     *
     * @param routeKey name of the route
     * @param routeTemplate template for the route, als defines parameters
     */
    public insertRoute(routeKey: K, routeTemplate: string) {
        const templatePairs = [
            ...parseTemplatePairs(
                routeTemplate,
                this.options.parameterPlaceholderRE,
            ),
        ];
        this.templatePairs.set(routeKey, templatePairs);
        const leafNode = this.rootNode.insert(routeKey, templatePairs);
        this.leafNodes.set(routeKey, leafNode);
        return this;
    }

    /**
     * @description
     * Match the path against a known routes and parse the parameters in it
     *
     * @param path path to match
     * @returns tuple with the route name or null if no route found. Then the parameters
     */
    public parseRoute(path: string): [K | null, Record<string, string>] {
        const parameters: Record<string, string> = {};

        const [routeKey, parameterValues] = this.rootNode.parse(
            path,
            this.options.maximumParameterValueLength,
        );

        if (routeKey == null) {
            return [null, {}];
        }

        const templatePairs = this.templatePairs.get(routeKey);
        if (templatePairs == null) {
            // this never happens
            return [null, {}];
        }

        for (let index = 0; index < parameterValues.length; index++) {
            const [, parameterName] = templatePairs[index + 1];
            if (parameterName == null) {
                // this never happens
                return [null, {}];
            }
            const parameterValue = parameterValues[index];
            parameters[parameterName] =
                this.options.parameterValueDecoder(parameterValue);
        }

        return [routeKey, parameters];
    }

    /**
     * @description
     * Convert a route to a path string.
     *
     * @param routeKey route to stringify
     * @param routeParameters parameters to include in the path
     * @returns string representing the route or null if the route is not found by name
     */
    public stringifyRoute(
        routeKey: K,
        routeParameters: Record<string, string> = {},
    ): string | null {
        let result = "";
        const templatePairs = this.templatePairs.get(routeKey);
        if (templatePairs == null) {
            return null;
        }
        for (let index = 0; index < templatePairs.length; index++) {
            const [parameterAnchor, parameterName] = templatePairs[index];
            if (parameterName != null) {
                const parameterValue = routeParameters[parameterName];
                result += this.options.parameterValueEncoder(parameterValue);
            }
            result += parameterAnchor;
        }
        return result;
    }

    public saveToJSON(): RouteNodeJson<K> {
        const json = this.rootNode.toJSON();
        return json;
    }

    public loadFromJSON(json: RouteNodeJson<K>) {
        const node = RouteNode.fromJSON(json);
        this.rootNode = node;
        return this;
    }
}
