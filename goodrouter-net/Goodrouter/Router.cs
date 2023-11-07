using System.Text.RegularExpressions;

/// <summary>
/// This is the router!
/// </summary>
public class Router<K> where K : notnull
{
    private RouteNode<K> rootNode = new RouteNode<K>();
    private readonly Dictionary<K, RouteNode<K>> leafNodes = new Dictionary<K, RouteNode<K>>();

    private Regex parameterPlaceholderRE = new Regex("\\{(.*?)\\}");
    private int maximumParameterValueLength = 20;

    private Func<string, string> parameterValueEncoder =
        (string decodedValue) => Uri.EscapeDataString(decodedValue);

    private Func<string, string> parameterValueDecoder =
        (string encodedValue) => Uri.UnescapeDataString(encodedValue);

    /// <summary>
    /// Set the maximum length of an encoded parameter value
    /// </summary>
    /// <param name="value">
    /// The maximum length
    /// </param>
    /// <returns>
    /// Router object, so you can chain!
    /// </returns>
    public Router<K> SetMaximumParameterValueLength(int value)
    {
        maximumParameterValueLength = value;
        return this;
    }

    /// <summary>
    /// Set the regular expression that will be used to parse placeholders from a route template
    /// </summary>
    /// <param name="value">
    /// The regular expression
    /// </param>
    /// <returns>
    /// Router object, so you can chain!
    /// </returns>
    public Router<K> SetParameterPlaceholderRE(Regex value)
    {
        parameterPlaceholderRE = value;
        return this;
    }

    /// <summary>
    /// Sets the function that will be used to encode parameter values
    /// </summary>
    /// <param name="value">
    /// Function to use
    /// </param>
    /// <returns>
    /// Router object, so you can chain!
    /// </returns>
    public Router<K> SetParameterValueEncoder(Func<string, string> value)
    {
        parameterValueEncoder = value;
        return this;
    }

    /// <summary>
    /// Sets the function that will be used when decoding parameter values
    /// </summary>
    /// <param name="value">
    /// Function to use
    /// </param>
    /// <returns>
    /// Router object, so you can chain!
    /// </returns>
    public Router<K> SetParameterValueDecoder(Func<string, string> value)
    {
        parameterValueDecoder = value;
        return this;
    }

    /// <summary>
    /// Adds a new route
    /// </summary>
    /// <param name="routeKey">
    /// name of the route
    /// </param>
    /// <param name="routeTemplate">
    /// template for the route, als defines parameters
    /// </param>
    public Router<K> InsertRoute(
        K routeKey,
        string routeTemplate
    )
    {
        var leafNode = this.rootNode.Insert(
            routeKey,
            routeTemplate,
            this.parameterPlaceholderRE
        );
        this.leafNodes.Add(routeKey, leafNode);
        return this;
    }

    /// <summary>
    /// Match the path against one of the provided routes and parse the parameters in it
    /// </summary>
    /// <param name="path">
    /// path to match
    /// </param>
    /// <returns>
    /// route that is matches to the path or null if no match is found
    /// </returns>
    public (K?, IReadOnlyDictionary<string, string>) ParseRoute(
        string path
    )
    {
        var parameters = new Dictionary<string, string>();

        var (routeKey, parameterNames, parameterValues) = this.rootNode.Parse(
            path,
            this.maximumParameterValueLength
        );

        for (var index = 0; index < parameterNames.Count; index++)
        {
            var parameterName = parameterNames[index];
            var parameterValue = parameterValues[index];
            parameters[parameterName] = parameterValueDecoder(parameterValue);
        }

        return (
            routeKey,
            parameters
        );
    }

    /// <summary>
    /// Convert a route to a path string.
    /// </summary>
    /// <param name="routeKey">
    /// route to stringify
    /// </param>
    /// <returns>
    /// string representing the route or null if the route is not found by name
    /// </returns>
    public string? StringifyRoute(
        K routeKey
    )
    {
        return StringifyRoute(routeKey, new Dictionary<string, string>());
    }
    /// <summary>
    /// Convert a route to a path string.
    /// </summary>
    /// <param name="routeKey">
    /// route to stringify
    /// </param>
    /// <param name="routeParameters">
    /// parameters for the route
    /// </param>
    /// <returns>
    /// string representing the route or null if the route is not found by name
    /// </returns>
    public string? StringifyRoute(
        K routeKey,
        IReadOnlyDictionary<string, string> routeParameters
    )
    {
        var node = this.leafNodes[routeKey];
        if (node == null) return null;

        var parameterValues = new List<string>();
        foreach (var parameterName in node.RouteParameterNames)
        {
            var parameterValue = routeParameters[parameterName];
            parameterValues.Add(parameterValueEncoder(parameterValue));
        }

        return node.Stringify(parameterValues);
    }

}

