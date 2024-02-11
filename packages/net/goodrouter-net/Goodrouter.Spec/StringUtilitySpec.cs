using Xunit;

public class StringUtilitySpec
{

    [Fact]
    public void FindCommonPrefixLengthTest()
    {
        Assert.Equal(
            2,
          StringUtility.FindCommonPrefixLength("ab", "abc")
      );

        Assert.Equal(
            3,
            StringUtility.FindCommonPrefixLength("abc", "abc")
        );

        Assert.Equal(
            0,
            StringUtility.FindCommonPrefixLength("bc", "abc")
        );
    }

}
