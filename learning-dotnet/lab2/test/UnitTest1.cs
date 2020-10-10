using System;
using Xunit;

using FsCheck;

using sut;

namespace test
{
    public class UnitTest1
    {
        [Fact]
        public void Quickcheck(){
            Prop.ForAll<int>(i => Someclass.add1(i) == i + 1 )
                .QuickCheckThrowOnFailure();
        }
    }
}
