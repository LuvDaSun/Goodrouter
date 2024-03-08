using Xunit;

public class RouteNodeSpec
{

  [Fact]
  public void RouteNodeSortTest()
  {
    var routeNodes = new RouteNode<string>[]{
            new RouteNode<string>("aa"),
            new RouteNode<string>("xx"),
            new RouteNode<string>("aa", true),
            new RouteNode<string>("x")
        };

    var sortedRouteNodes = new SortedSet<RouteNode<string>>(routeNodes).ToArray();

    Assert.Equal(routeNodes, sortedRouteNodes);
  }
}
