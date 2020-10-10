package StochasticMapMovement;

import StochasticMapMovement.spacial.GameMap2D;
import StochasticMapMovement.spacial.Point2D;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.concurrent.TimeUnit;

public class Main {
    static Organism org = new Organism(new RandomMovementStrategy());
    static GameMap2D map = new GameMap2D(new ArrayList<>(Arrays.asList(org)), 5, 5);

    public static void nextTick()
    {
        // TODO: Extract to game class

        try
        {
            TimeUnit.SECONDS.sleep(1);
        }
        catch (InterruptedException e)
        {

        }
        System.out.print("\033[H\033[2J");
        System.out.flush();

        map.move(org);
        System.out.println(map.render());
    }

    public static void main(String[] args)
    {
        System.out.println(map.render());

        while(true)
        {
            nextTick();
        }
    }
}