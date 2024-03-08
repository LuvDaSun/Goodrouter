using System.Text.RegularExpressions;
using BenchmarkDotNet.Attributes;


public class RouterBenchSmall : RouterBenchBase
{
  public RouterBenchSmall() : base("small")
  {
  }
}

public class RouterBenchDocker : RouterBenchBase
{
  public RouterBenchDocker() : base("docker")
  {
  }
}

public class RouterBenchGithub : RouterBenchBase
{
  public RouterBenchGithub() : base("github")
  {
  }
}

public abstract class RouterBenchBase
{

  protected RouterBenchBase(string name)
  {
    templates = System.IO.File.ReadLines("" + name + ".txt").
       Where(line => line.Length > 0).
       ToArray();

    var allParameterNames = templates.
        SelectMany(
            template => TemplateUtility.ParseTemplateParts(template, parameterPlaceholderRE).
                Where((part, index) => index % 2 != 0)
        ).
        ToHashSet();

    allParameters = allParameterNames.
       Select((name, index) => (name, "p" + index)).
       ToDictionary(pair => pair.name, pair => pair.Item2);

    templateCount = templates.Count;

    router = new Router<string>();
    foreach (var template in templates)
    {
      router.InsertRoute(template, template);
    }

    paths = templates.
       Select(template =>
           {
             var path = router.StringifyRoute(template, allParameters);
             return path;
           }
       ).
       ToArray() as string[];
  }

  private Router<string> router;
  private Regex parameterPlaceholderRE = new Regex("\\{(.*?)\\}");
  private int index = 0;
  private readonly IList<string> paths;
  private readonly IList<string> templates;
  private readonly int templateCount;
  private readonly IReadOnlyDictionary<string, string> allParameters;

  [Benchmark]
  public void RouterParseBench()
  {
    var path = paths[index % templateCount];
    router.ParseRoute(path);

    index++;
  }

  [Benchmark]
  public void RouterStringifyBench()
  {
    var template = templates[index % templateCount];
    router.StringifyRoute(template, allParameters);

    index++;
  }

}

