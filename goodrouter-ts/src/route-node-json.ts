export interface RouteNodeJson<K extends string | number> {
    anchor: string;
    hasParameter: boolean;
    routeKey: K | null;
    children: RouteNodeJson<K>[];
}
