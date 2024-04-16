using Arch.Core;
using Arch.Core.Extensions;
using Godot;
using the_colony.Systems;

public partial class Pawn : CharacterBody2D
{
    private NextPosition _nextPos;
    private Entity entity;
    private NavigationAgent2D navAgent;

    private Vector2[] path;
    private Vector2 pos;

    private Vector2 targetVec;

    public override void _Ready()
    {
        _nextPos.nextPos = new Vector2(Position.X, Position.Y);

        entity = WorldManager.world.Create(
            new Position { pos = new Vector2(Position.X, Position.Y) },
            _nextPos,
            new TargetPosition { targetPos = new Vector2(Position.X, Position.Y) },
            new MoveSpeed { speed = 50 },
            new Velocity { Dx = 10, Dy = 10 },
            new NeedsPath(),
            new Hunger { current = 100, max = 100, rate = 3 },
            new Task { task = Tasks.Wander },
            new ChooseTask(),
            this as CharacterBody2D);
    }

    public override void _Draw()
    {
        base._Draw();
        return;

        if (path != null && path.Length > 0)
            for (var i = 0; i < path.Length - 1; ++i)
                DrawLine(ToLocal(path[i]), ToLocal(path[i + 1]), Colors.Red, 5);

        DrawCircle(pos, 10, Colors.Cyan);
        DrawCircle(targetVec, 12, Colors.Purple);
        DrawCircle(ToLocal(GetGlobalMousePosition()), 10, Colors.Orange);
    }

    public override void _Process(double delta)
    {
        return;
        var point = WorldManager.aStar.GetClosestPoint(GetGlobalMousePosition());

        path = WorldManager.aStar.GetPointPath(WorldManager.aStar.GetClosestPoint(Position), point);

        if (path.Length < 1)
            _nextPos.nextPos = WorldManager.aStar.GetPointPosition(WorldManager.aStar.GetClosestPoint(Position));
        else if (path.Length < 2)
            _nextPos.nextPos = ToLocal(path[0]);
        else
            _nextPos.nextPos = ToLocal(path[1]);

        targetVec = _nextPos.nextPos;
        pos = ToLocal(WorldManager.aStar.GetPointPosition(WorldManager.aStar.GetClosestPoint(Position)));

        entity.Set(_nextPos);

        //QueueRedraw();
    }
}