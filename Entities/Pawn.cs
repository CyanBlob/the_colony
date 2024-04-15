using Arch.Core;
using Arch.Core.Extensions;
using Godot;

public partial class Pawn : CharacterBody2D
{
    private Entity entity;
    private NavigationAgent2D navAgent;

    private Vector2[] path;
    private Vector2 pos;
    private TargetPosition targetPos;

    private Vector2 targetVec;

    public override void _Ready()
    {
        navAgent = GetNode<NavigationAgent2D>("NavigationAgent2D");

        targetPos.targetPos = new Vector2(Position.X, Position.Y);

        entity = WorldManager.world.Create(
            new Position { pos = new Vector2(Position.X, Position.Y) },
            targetPos,
            new MoveSpeed { speed = 10 },
            new Velocity { Dx = 10, Dy = 10 },
            this as CharacterBody2D,
            this);
    }

    public override void _Draw()
    {
        base._Draw();

        if (path != null && path.Length > 0)
            for (var i = 0; i < path.Length - 1; ++i)
                DrawLine(ToLocal(path[i]), ToLocal(path[i + 1]), Colors.Red, 5);

        DrawCircle(pos, 10, Colors.Cyan);
        DrawCircle(targetVec, 12, Colors.Purple);
        DrawCircle(ToLocal(GetGlobalMousePosition()), 10, Colors.Orange);
    }

    public override void _Process(double delta)
    {
        var point = WorldManager.aStar.GetClosestPoint(GetGlobalMousePosition());

        path = WorldManager.aStar.GetPointPath(WorldManager.aStar.GetClosestPoint(Position), point);

        if (path.Length < 1)
            targetPos.targetPos = Position;
        else if (path.Length < 2)
            targetPos.targetPos = ToLocal(path[0]);
        else
            targetPos.targetPos = ToLocal(path[1]);

        targetVec = targetPos.targetPos;
        pos = ToLocal(WorldManager.aStar.GetPointPosition(WorldManager.aStar.GetClosestPoint(Position)));

        entity.Set(targetPos);

        //QueueRedraw();
    }
}