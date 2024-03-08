using System.Text.RegularExpressions;

internal static class TemplateUtility
{

  public static IEnumerable<string> ParseTemplateParts(
      string routeTemplate,
      Regex parameterPlaceholderRE
  )
  {
    var offsetIndex = 0;
    foreach (Match match in parameterPlaceholderRE.Matches(routeTemplate))
    {
      yield return routeTemplate.Substring(
          offsetIndex,
          match.Index - offsetIndex
      );
      yield return match.Groups[1].Value;
      offsetIndex = match.Index + match.Length;
    }
    yield return routeTemplate.Substring(offsetIndex);
  }

  public static IEnumerable<(string, string?)> ParseTemplatePairs(
      string routeTemplate,
      Regex parameterPlaceholderRE
  )
  {
    var parts = ParseTemplateParts(routeTemplate, parameterPlaceholderRE);

    var index = 0;
    string? parameter = null;
    foreach (var part in parts)
    {
      if (index % 2 == 0)
      {
        yield return (part, parameter);
      }
      else
      {
        parameter = part;
      }
      index++;
    }

  }
}
