package StochasticMapMovement.spacial;

import StochasticMapMovement.Organism;
import StochasticMapMovement.spacial.interfaces.GameMap;
import StochasticMapMovement.spacial.interfaces.MovementDirection;
import StochasticMapMovement.spacial.interfaces.Point;

import java.util.*;

public class GameMap2D implements GameMap
{
    private final Map<Organism, Point> positions;
    private final int width;
    private final int height;

    // TODO: Generalize
    public List<MovementDirection> availableDirections(Point2D position)
    {
        List<MovementDirection> availableDirections = new ArrayList<>();

        // TODO: IsInMap, run simulation and then output thoose who pass the test
        if (position.x > 1)
            availableDirections.add(MovementDirection2D.Left);

        if (position.x < width - 1)
            availableDirections.add(MovementDirection2D.Right);

        if (position.y > 1)
            availableDirections.add(MovementDirection2D.Up);

        if (position.y < height - 1)
            availableDirections.add(MovementDirection2D.Down);

        return availableDirections;
    }

    @Override
    public void move(Organism o)
    {
        // TODO: Points cannot be added, points can be move by a vector
        java.util.Map<MovementDirection2D, Point2D> movementResolutionTable = new HashMap<>();
        movementResolutionTable.put(MovementDirection2D.Left, new Point2D(-1, 0));
        movementResolutionTable.put(MovementDirection2D.Down, new Point2D(0, 1));
        movementResolutionTable.put(MovementDirection2D.Right, new Point2D(1, 0));
        movementResolutionTable.put(MovementDirection2D.Up, new Point2D(0, -1));

        Point2D movementDelta = movementResolutionTable.get((MovementDirection2D) o.nextMoveDirection(this.availableDirections((Point2D) positions.get(o))));

        positions.compute(o, (__,p) -> ((Point2D) p).add(movementDelta));
    }

    public String render()
    {
        char fillValue = ' ';
        char borderValue = '#';
        char rowDelimiter = '\n';

        char[][] stringMap = new char[this.width + 2][this.height + 2];

        // preinitialize the map
        for (char[] row : stringMap)
        {
            Arrays.fill(row, fillValue);
        }

        // redner frame
        for (char[] row : stringMap)
        {
            row[0] = borderValue;
            row[row.length - 1] = borderValue;
        }

        int index = 0;
        for (char q: stringMap[0])
        {
            stringMap[0][index] = borderValue;
            index++;
        }

        index = 0;
        for (char q : stringMap[stringMap.length - 1])
        {
            stringMap[stringMap.length - 1][index] = borderValue;
            index++;
        }

        // render obejcts
        for (Map.Entry<Organism, Point> entry : positions.entrySet())
        {
            Point2D p = (Point2D) entry.getValue();

            // TODO: XD Figure out the axis
            stringMap[p.y + 1][p.x + 1]='$';
        }

        StringBuilder sb = new StringBuilder();

        for (char[] row : stringMap)
        {
            for (char i: row)
            {
                sb.append(i);
            }

            sb.append(rowDelimiter);
        }
        return sb.toString();
    }

    public GameMap2D(List<Organism> organisms, int width, int height)
    {
        this.positions = organisms
                .stream()
                .map(o ->
                {
                    Map<Organism, Point> m = new HashMap<>();
                    // TODO: Extract placement strategy
                    m.put(o, new Point2D(0,0));
                    return m;
                })
                .reduce(new HashMap<>(), (acc, mo) -> { acc.putAll(mo); return acc; }
                );

        this.width = width;
        this.height = height;
    }
}
