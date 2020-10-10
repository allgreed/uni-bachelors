package StochasticMapMovement;

import StochasticMapMovement.spacial.MovementDirection2D;
import StochasticMapMovement.spacial.interfaces.MovementDirection;
import StochasticMapMovement.spacial.interfaces.MovementStrategy;
import StochasticMapMovement.spacial.Point2D;

import java.util.HashMap;
import java.util.List;

public class Organism
{
    private final MovementStrategy movementStrategy;

    // initiative
    // movementSpeed
    // TODO: Implement

    public MovementDirection nextMoveDirection(List<MovementDirection> availableDirections)
    {
        return this.movementStrategy.next(availableDirections);
    }

    public Organism(MovementStrategy movementStrategy)
    {
        this.movementStrategy = movementStrategy;
    }
}
