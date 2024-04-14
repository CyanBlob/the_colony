using Arch.Core;
using Arch.Core.Extensions;
using Godot;

public partial class Pawn : CharacterBody2D
{
    private Entity entity;
    private NavigationAgent2D navAgent;
    private TargetPosition targetPos;

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

    // Called every frame. 'delta' is the elapsed time since the previous frame.
    public override void _Process(double delta)
    {
        navAgent.TargetPosition = GetGlobalMousePosition();
        targetPos.targetPos = navAgent.GetNextPathPosition() - GlobalPosition;

        entity.Set(targetPos);
    }
}