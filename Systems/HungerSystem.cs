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

    [Query]
    public void GetHungry([Data] in double time, ref Hunger hunger)
    {
        hunger.current -= hunger.rate * (float)time;

        if (hunger.current < 0) hunger.current = 0;
    }
}