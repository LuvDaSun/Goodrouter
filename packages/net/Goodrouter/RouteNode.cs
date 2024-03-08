using System.Text.RegularExpressions;

internal class RouteNode<K> : IComparable<RouteNode<K>>, IEquatable<RouteNode<K>> where K : notnull
{
  public string Anchor { get; private set; }
  public bool HasParameter { get; private set; }
  public K? RouteKey { get; private set; }

  public IList<string> RouteParameterNames { get; private set; }

  private SortedSet<RouteNode<K>> children = new SortedSet<RouteNode<K>>();
  public IReadOnlySet<RouteNode<K>> Children
  {
    get
    {
      return this.children;
    }
  }

  private void AddChild(RouteNode<K> node)
  {
    if (node.parent != null)
    {
      throw new ArgumentException("node already has a parent", "node");
    }

    node.parent = this;
    this.children.Add(node);
  }

  private void RemoveChild(RouteNode<K> node)
  {
    node.parent = null;
    this.children.Remove(node);
  }



  private RouteNode<K>? parent;
  public RouteNode<K>? Parent
  {
    get
    {
      return this.parent;
    }
  }


  public RouteNode(
      string anchor = "",
      bool hasParameter = false,
      K? routeKey = default(K?)
  ) : this(
      anchor,
      hasParameter,
      routeKey,
      new string[] { }
  )
  {
  }
  public RouteNode(
      string anchor,
      bool hasParameter,
      K? routeKey,
      IList<string> routeParameterNames
  )
  {
    this.Anchor = anchor;
    this.HasParameter = hasParameter;
    this.RouteKey = routeKey;
    this.RouteParameterNames = routeParameterNames;
  }

  public RouteNode<K> Insert(
      K routeKey,
      string routeTemplate,
      Regex parameterPlaceholderRE
  )
  {
    var pairs =
        TemplateUtility.ParseTemplatePairs(routeTemplate, parameterPlaceholderRE).ToArray();

    var routeParameterNames = pairs.
        Select(pair =>
        {
          var (anchor, parameterName) = pair;
          return parameterName;
        }).
        Where(parameterName => parameterName != null).
        ToArray() as string[];

    var currentNode = this;
    for (var index = 0; index < pairs.Length; index++)
    {
      var (anchor, parameterName) = pairs[index];
      var hasParameter = parameterName != null;

      var (commonPrefixLength, childNode) =
          currentNode.FindSimilarChild(anchor, hasParameter);

      currentNode = currentNode.Merge(
          childNode,
          anchor,
          hasParameter,
          index == pairs.Length - 1 ? routeKey : default(K?),
          routeParameterNames,
          commonPrefixLength
      );

    }

    return currentNode;
  }

  public (K?, IList<string>, IList<string>) Parse(
      string path,
      int maximumParameterValueLength
  )
  {
    var parameterValues = new List<string>();

    if (this.HasParameter)
    {
      if (path.Length == 0)
      {
        return (default(K?), new string[] { }, new string[] { });
      }

      var index = this.Anchor.Length == 0 ?
      path.Length :
      path.Substring(0, Math.Min(
          maximumParameterValueLength + this.Anchor.Length,
          path.Length
      )).
          IndexOf(this.Anchor);

      if (index < 0)
      {
        return (default(K?), new string[] { }, new string[] { });
      }

      var value = path.Substring(0, index);

      path = path.Substring(index + this.Anchor.Length);

      parameterValues.Add(value);
    }
    else
    {
      if (!path.StartsWith(this.Anchor))
      {
        return (default(K?), new string[] { }, new string[] { });
      }

      path = path.Substring(this.Anchor.Length);
    }

    foreach (var childNode in this.children)
    {
      var (childRouteKey, childRouteParameterNames, childParameterValues) = childNode.Parse(
          path,
          maximumParameterValueLength
      );

      if (!Object.Equals(childRouteKey, default(K?)))
      {
        return (
            childRouteKey,
            childRouteParameterNames,
            parameterValues.Concat(childParameterValues).ToArray()
        );
      }
    }

    if (!Object.Equals(this.RouteKey, default(K?)) && path.Length == 0)
    {
      return (
          this.RouteKey,
          this.RouteParameterNames,
          parameterValues
      );
    }

    return (default(K?), new string[] { }, new string[] { });
  }

  public string Stringify(
      IList<string> parameterValues
  )
  {
    var parameterIndex = parameterValues.Count;
    var path = "";
    var currentNode = this;
    while (currentNode != null)
    {
      path = currentNode.Anchor + path;
      if (currentNode.HasParameter)
      {
        parameterIndex--;
        var value = parameterValues[parameterIndex];
        path = value + path;
      }
      currentNode = currentNode.Parent;
    }
    return path;
  }


  private RouteNode<K> Merge(
      RouteNode<K>? childNode,
      string anchor,
      bool hasParameter,
      K? routeKey,
      IList<string> routeParameterNames,
      int commmonPrefixLength
  )
  {
    if (childNode == null)
    {
      return this.MergeNew(
          anchor,
          hasParameter,
          routeKey,
          routeParameterNames
      );
    }

    var commonPrefix = childNode.Anchor.Substring(0, commmonPrefixLength);

    if (childNode.Anchor == anchor)
    {
      return this.MergeJoin(
          childNode,
          routeKey,
          routeParameterNames
      );
    }
    else if (childNode.Anchor == commonPrefix)
    {
      return this.MergeAddToChild(
          childNode,
          anchor,
          hasParameter,
          routeKey,
          routeParameterNames,
          commmonPrefixLength
      );
    }
    else if (anchor == commonPrefix)
    {
      return this.MergeAddToNew(
          childNode,
          anchor,
          hasParameter,
          routeKey,
          routeParameterNames,
          commmonPrefixLength
      );
    }
    else
    {
      return this.MergeIntermediate(
          childNode,
          anchor,
          hasParameter,
          routeKey,
          routeParameterNames,
          commmonPrefixLength
      );
    }
  }

  private RouteNode<K> MergeNew(
      string anchor,
      bool hasParameter,
      K? routeKey,
      IList<string> routeParameterNames
  )
  {
    var newNode = new RouteNode<K>(
        anchor,
        hasParameter,
        routeKey,
        routeParameterNames
    );
    this.AddChild(newNode);
    return newNode;
  }

  private RouteNode<K> MergeJoin(
      RouteNode<K> childNode,
      K? routeKey,
      IList<string> routeParameterNames
  )
  {
    if (
        !Object.Equals(childNode.RouteKey, default(K?)) &&
        !Object.Equals(routeKey, default(K?))
    )
    {
      throw new Exception("ambiguous route");
    }

    if (Object.Equals(childNode.RouteKey, default(K?)))
    {
      childNode.RouteKey = routeKey;
      childNode.RouteParameterNames = routeParameterNames;
    }

    return childNode;
  }

  private RouteNode<K> MergeIntermediate(
      RouteNode<K> childNode,
      string anchor,
      bool hasParameter,
      K? routeKey,
      IList<string> routeParameterNames,
      int commmonPrefixLength
  )
  {
    this.RemoveChild(childNode);

    var newNode = new RouteNode<K>(
        anchor.Substring(commmonPrefixLength),
        false,
        routeKey,
        routeParameterNames
    );

    childNode.Anchor = childNode.Anchor.Substring(commmonPrefixLength);
    childNode.HasParameter = false;

    var intermediateNode = new RouteNode<K>(
        anchor.Substring(0, commmonPrefixLength),
        hasParameter
    );
    intermediateNode.AddChild(childNode);
    intermediateNode.AddChild(newNode);

    this.AddChild(intermediateNode);

    return newNode;
  }

  private RouteNode<K> MergeAddToChild(
      RouteNode<K> childNode,
      string anchor,
      bool hasParameter,
      K? routeKey,
      IList<string> routeParameterNames,
      int commmonPrefixLength
  )
  {
    anchor = anchor.Substring(commmonPrefixLength);
    hasParameter = false;

    var (commonPrefixLength2, childNode2) =
        childNode.FindSimilarChild(anchor, hasParameter);

    return childNode.Merge(
        childNode2,
        anchor,
        hasParameter,
        routeKey,
        routeParameterNames,
        commonPrefixLength2
    );
  }

  private RouteNode<K> MergeAddToNew(
      RouteNode<K> childNode,
      string anchor,
      bool hasParameter,
      K? routeKey,
      IList<string> routeParameterNames,
      int commmonPrefixLength
  )
  {
    var newNode = new RouteNode<K>(
        anchor,
        hasParameter,
        routeKey,
        routeParameterNames
    );
    this.AddChild(newNode);

    this.RemoveChild(childNode);

    childNode.Anchor = childNode.Anchor.Substring(commmonPrefixLength);
    childNode.HasParameter = false;

    newNode.AddChild(childNode);

    return newNode;
  }

  private (int, RouteNode<K>?) FindSimilarChild(
      string anchor,
      bool hasParameter
  )
  {
    foreach (var childNode in this.children)
    {
      if (childNode.HasParameter != hasParameter)
      {
        continue;
      }

      var commonPrefixLength = StringUtility.FindCommonPrefixLength(anchor, childNode.Anchor);
      if (commonPrefixLength == 0)
      {
        continue;
      }

      return (commonPrefixLength, childNode);
    }

    return (0, null);
  }

  public int CompareTo(RouteNode<K>? other)
  {
    if (other == null)
    {
      return 1;
    }

    {
      var compared = this.Anchor.Length.CompareTo(other.Anchor.Length);
      if (compared != 0)
      {
        return 0 - compared;
      }
    }

    {
      var compared = this.HasParameter.CompareTo(other.HasParameter);
      if (compared != 0)
      {
        return compared;
      }
    }

    {
      var compared = this.Anchor.CompareTo(other.Anchor);
      if (compared != 0)
      {
        return compared;
      }
    }

    return 0;
  }

  public bool Equals(RouteNode<K>? other)
  {
    if (other == null) return false;

    return this.Anchor == other.Anchor &&
        this.HasParameter == other.HasParameter &&
        Object.Equals(this.RouteKey, other.RouteKey);
  }

}
