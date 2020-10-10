package StochasticMapMovement.spacial;

public class Point2D implements StochasticMapMovement.spacial.interfaces.Point
{
    public final int x;
    public final int y;

    public Point2D(int x, int y)
    {
        this.x = x;
        this.y = y;
    }

    public Point2D add(Point2D other)
    {
        return new Point2D(this.x + other.x, this.y + other.y);
    }
}
