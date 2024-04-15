using Godot;

namespace the_colony.Entities;

public partial class Food : Node2D
{
    public Food()
    {
        WorldManager.world.Create(
            new Position { pos = new Vector2(Position.X, Position.Y) },
            new MoveSpeed { speed = 10 },
            this as Node2D,
            this);
    }
}