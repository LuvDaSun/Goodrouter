using System.Text.RegularExpressions;
using Xunit;

enum Route
{
    NotFound,
    A,
    B,
    C,
    D
}

public class RouterSpec
{
    private Regex parameterPlaceholderRE = new Regex("\\{(.*?)\\}");

    [Fact]
    public void RouterTest()
    {
        var router = new Router<Route>();

        router.InsertRoute(Route.A, "/a");
        router.InsertRoute(Route.B, "/b/{x}");
        router.InsertRoute(Route.C, "/b/{x}/c");
        router.InsertRoute(Route.D, "/b/{x}/d");

        {
            var (routeKey, routeParameters) = router.ParseRoute("/not-found");
            Assert.Equal(Route.NotFound, routeKey);
            Assert.Equal(new Dictionary<string, string>() { }, routeParameters);
        }

        {
            var (routeKey, routeParameters) = router.ParseRoute("/a");
            Assert.Equal(Route.A, routeKey);
            Assert.Equal(new Dictionary<string, string>() { }, routeParameters);
        }

        {
            var (routeKey, routeParameters) = router.ParseRoute("/b/x");
            Assert.Equal(Route.B, routeKey);
            Assert.Equal(new Dictionary<string, string>() { { "x", "x" } }, routeParameters);
        }

        {
            var (routeKey, routeParameters) = router.ParseRoute("/b/y/c");
            Assert.Equal(Route.C, routeKey);
            Assert.Equal(new Dictionary<string, string>() { { "x", "y" } }, routeParameters);
        }

        {
            var (routeKey, routeParameters) = router.ParseRoute("/b/z/d");
            Assert.Equal(Route.D, routeKey);
            Assert.Equal(new Dictionary<string, string>() { { "x", "z" } }, routeParameters);
        }

    }

    [Fact]
    public void RouterTestReadme()
    {
        var router = new Router<string>();

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
            Assert.Equal("product-detail", routeKey);
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

    }

    [Fact]
    public void RouterTestSmall()
    {
        RouterTestTemplates("small");
    }

    [Fact]
    public void RouterTestDocker()
    {
        RouterTestTemplates("docker");
    }

    [Fact]
    public void RouterTestGithub()
    {
        RouterTestTemplates("github");
    }

    private void RouterTestTemplates(string name)
    {
        var templates = System.IO.File.ReadLines("" + name + ".txt").
            Where(line => line.Length > 0).
            ToArray();

        var allParameterNames = templates.
            SelectMany(
                template => TemplateUtility.ParseTemplateParts(template, parameterPlaceholderRE).
                    Where((part, index) => index % 2 != 0)
            ).
            ToHashSet();

        var allParameters = allParameterNames.
            Select((name, index) => (name, "p" + index)).
            ToDictionary(pair => pair.name, pair => pair.Item2);

        var templateCount = templates.Length;

        var router = new Router<string>();
        foreach (var template in templates)
        {
            router.InsertRoute(template, template);
        }

        var paths = templates.
            Select(template =>
                {
                    var path = router.StringifyRoute(template, allParameters);
                    return path;
                }
            ).
            ToArray() as string[];

        for (var index = 0; index < templateCount; index++)
        {
            var path = paths[index];
            var template = templates[index];

            var (routeKey, routeParameters) = router.ParseRoute(path);

            var expectedParameters = routeParameters.Keys.
                Select(name => (name, allParameters[name])).
                ToDictionary(pair => pair.name, pair => pair.Item2);

            Assert.Equal(template, routeKey);
            Assert.Equal(expectedParameters, routeParameters);
        }
    }
}
