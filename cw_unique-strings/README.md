Given a string that contains only letters, you have to find out the number of unique strings (including the string itself) that can be produced by re-arranging the letters of the string. Strings are case insensitive.

HINT: Generating all the unique strings and calling length on that isn't a great solution for this problem. It can be done a lot faster...

Examples
uniqcount("AB") = 2      # "AB", "BA"
uniqcount("ABC") = 6     # "ABC", "ACB", "BAC", "BCA", "CAB", "CBA"
uniqcount("ABA") = 3     # "AAB", "ABA", "BAA"
uniqcount("ABBb") = 4    # "ABBB", "BABB", "BBAB", "BBBA"
uniqcount("AbcD") = 24   # "ABCD", etc.

https://www.codewars.com/kata/586c7cd3b98de02ef60001ab 