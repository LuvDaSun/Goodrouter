internal static class StringUtility
{
    public static int FindCommonPrefixLength(
        string stringLeft,
        string stringRight
    )
    {
        var length = Math.Min(stringLeft.Length, stringRight.Length);
        int index;
        for (index = 0; index < length; index++)
        {
            var charLeft = stringLeft[index];
            var charRight = stringRight[index];
            if (charLeft != charRight)
            {
                break;
            }
        }
        return index;
    }

}
