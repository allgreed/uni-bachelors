package StochasticMapMovement.spacial.interfaces;

import StochasticMapMovement.Organism;
import StochasticMapMovement.spacial.MovementDirection2D;
import java.util.List;

public interface GameMap
{
    // TODO: Extract to view ?
    String render();

    void move(Organism o);
    //public List<MovementDirection> availableDirections(Point position);
}
