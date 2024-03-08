using System.Text.RegularExpressions;
using Xunit;

public class TemplateUtilitySpec
{
  private Regex parameterPlaceholderRE = new Regex("\\{(.*?)\\}");

  [Fact]
  public void ParseTemplatePartsTest()
  {
    {
      var parts = TemplateUtility.ParseTemplateParts("/a/{b}/{c}", parameterPlaceholderRE).ToArray();

      Assert.Equal(
          new string[] { "/a/", "b", "/", "c", "" },
          parts
      );
    }

    {
      var parts = TemplateUtility.ParseTemplateParts("/a/{b}/{c}/", parameterPlaceholderRE).ToArray();

      Assert.Equal(
          new string[] { "/a/", "b", "/", "c", "/" },
          parts
      );
    }

    {
      var parts = TemplateUtility.ParseTemplateParts("", parameterPlaceholderRE).ToArray();

      Assert.Equal(
          new string[] { "" },
          parts
      );
    }

  }

  [Fact]
  public void ParseTemplatePairsTest()
  {
    {
      var parts = TemplateUtility.ParseTemplatePairs("/a/{b}/{c}", parameterPlaceholderRE).ToArray();

      Assert.Equal(
          new (string, string?)[] { ("/a/", null), ("/", "b"), ("", "c") },
          parts
      );
    }

    {
      var parts = TemplateUtility.ParseTemplatePairs("/a/{b}/{c}/", parameterPlaceholderRE).ToArray();

      Assert.Equal(
          new (string, string?)[] { ("/a/", null), ("/", "b"), ("/", "c") },
          parts
      );
    }

    {
      var parts = TemplateUtility.ParseTemplatePairs("", parameterPlaceholderRE).ToArray();

      Assert.Equal(
          new (string, string?)[] { ("", null) },
          parts
      );
    }
  }

}
