package StochasticMapMovement;

import StochasticMapMovement.spacial.interfaces.MovementDirection;
import StochasticMapMovement.spacial.interfaces.MovementStrategy;
import StochasticMapMovement.spacial.interfaces.NullMovementDirection;

import java.util.List;
import java.util.Random;

public class RandomMovementStrategy implements MovementStrategy
{
    @Override
    public MovementDirection next(List<MovementDirection> availableDirections)
    {
        availableDirections.remove(NullMovementDirection.Null);

        Random random = new Random();
        int randomDirectionIndex = random.nextInt(availableDirections.size());

        MovementDirection nextMoveDirection  = availableDirections.get(randomDirectionIndex);

        return nextMoveDirection;
    }
}
