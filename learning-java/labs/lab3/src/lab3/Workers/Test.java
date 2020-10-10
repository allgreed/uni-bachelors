package lab3.Workers;

import java.util.ArrayList;

public class Test
{
    public static void main(String[] args)
    {
        class X {
            X(){

            }

            X ble(int x)
            {
                System.out.println(x);
                return this;
            }
        }

        ArrayList<X> fe = new ArrayList<X>();
        fe.add(new X());
        fe.add(new X());
        fe.add(new X());

        fe.stream()
                .map(x -> x.ble(4));
    }
}
