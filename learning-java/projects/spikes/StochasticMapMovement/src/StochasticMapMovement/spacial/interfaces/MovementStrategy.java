package StochasticMapMovement.spacial.interfaces;

import StochasticMapMovement.spacial.MovementDirection2D;

import java.util.List;

public interface MovementStrategy
{
    MovementDirection next(List<MovementDirection> availableDirections);
}
