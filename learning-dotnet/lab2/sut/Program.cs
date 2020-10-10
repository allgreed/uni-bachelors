using System;

namespace sut
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine(Someclass.add1(1));
        }
    }
}

public static class Someclass
{
    public static int add1(int x)
    {
        if ( x > 100)
        //if (x % 4 != 3 && x > 100)
            return 7;

        return x + 1;
    }
}
