export interface RouteNodeJson<K extends string | number> {
    anchor: string;
    hasParameter: boolean;
    routeKey: K | null;
    children: RouteNodeJson<K>[];
}

export interface RouterJson<K extends string | number> {
    rootNode?: RouteNodeJson<K>;
    templatePairs?: Array<
        readonly [K, Array<readonly [string, string | null]>]
    >;
}
