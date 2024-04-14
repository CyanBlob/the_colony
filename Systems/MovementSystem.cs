using Arch.Core;
using Arch.System;
using Godot;

// Components ( ignore the formatting, this saves space )
public struct Position
{
    public Vector2 pos;
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

// BaseSystem provides several useful methods for interacting and structuring systems
public partial class MovementSystem : BaseSystem<World, double>
{
    public MovementSystem(World world) : base(world)
    {
    }

    // Generates a query and calls that one automatically on BaseSystem.Update
    [Query]
    public void Move([Data] in double time, ref Position pos, ref TargetPosition targetPos, ref MoveSpeed speed,
        ref Pawn pawn)
    {
        var direction = targetPos.targetPos;
        direction = direction.Normalized();
        direction *= speed.speed;

        pawn.Velocity = direction;

        pawn.MoveAndSlide();
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