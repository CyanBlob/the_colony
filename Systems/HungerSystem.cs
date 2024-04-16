using System.Diagnostics;
using Arch.Core;
using Arch.System;

namespace the_colony.Systems;

public struct Hunger
{
    public float current;
    public float max;
    public float rate;
}

public struct ProvidesFood
{
    public float amount;
}

public partial class HungerSystem : BaseSystem<World, double>
{
    public HungerSystem(World world) : base(world)
    {
    }

    /*[Query]
    [All(typeof(Eat))]
    private static void FindFood(ref TargetPosition targetPos)
    {
        var query = new QueryDescription().WithAll<ProvidesFood>();
        WorldManager.world.Query(in query,
            (ref ProvidesFood food, ref Position pos) => { Debug.Print($"Found food at: {pos.pos}"); });
    }*/

    [Query]
    private static void GetHungry([Data] in double time, ref Hunger hunger)
    {
        hunger.current -= hunger.rate * (float)time;

        if (hunger.current < 0) hunger.current = 0;

        Debug.Print($"Hunger: {hunger.current}");
    }
}