using Arch.Core;
using Arch.Core.Extensions;
using Arch.System;
using Godot;
using the_colony.Systems;

// Components ( ignore the formatting, this saves space )
public struct Position
{
    public Vector2 pos;
}

public struct NextPosition
{
    public Vector2 nextPos;
}

public struct TargetPosition
{
    public Vector2 targetPos;
}

public struct MoveSpeed
{
    public float speed;
}

public struct Velocity
{
    public float Dx;
    public float Dy;
}

public struct NeedsPath
{
}

// BaseSystem provides several useful methods for interacting and structuring systems
internal partial class MovementSystem : BaseSystem<World, double>
{
    public MovementSystem(World world) : base(world)
    {
    }

    // Generates a query and calls that one automatically on BaseSystem.Update
    [Query]
    private static void Move(ref Position pos, ref NextPosition nextPos, ref MoveSpeed speed,
        ref CharacterBody2D pawn)
    {
        if ((pos.pos - nextPos.nextPos).Length() < 1) return;

        var direction = nextPos.nextPos;
        direction = direction.Normalized();
        direction *= speed.speed;

        pawn.Velocity = direction;

        pawn.MoveAndSlide();
    }

    [Query]
    private static void Wander(ref CharacterBody2D character, ref TargetPosition targetPos, in Entity entity)
    {
        entity.Add(new ChooseTask());
    }

    // TODO: shouldn't run unless there's a change to the path
    [Query]
    private static void SetPath(ref CharacterBody2D character,
        ref NextPosition nextPos,
        ref TargetPosition targetPos)
    {
        var point = WorldManager.aStar.GetClosestPoint(targetPos.targetPos);

        var path = WorldManager.aStar.GetPointPath(WorldManager.aStar.GetClosestPoint(character.Position), point);

        if (path.Length < 1)
            nextPos.nextPos =
                WorldManager.aStar.GetPointPosition(WorldManager.aStar.GetClosestPoint(character.Position));
        else if (path.Length < 2)
            nextPos.nextPos = character.ToLocal(path[0]);
        else
            nextPos.nextPos = character.ToLocal(path[1]);
    }

    // Generates and filters a query and calls that one automatically on BaseSystem.Update in order
    /*[Query]
    [All<Player, Mob>]
    [Any<Idle, Moving>]
    [None<Alive>] // Attributes also accept non generics :)
    public void ResetVelocity(ref Velocity vel)
    {
        vel = new Velocity { X = 0, Y = 0 };
    }*/
}