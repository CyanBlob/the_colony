using Arch.Core;
using Arch.Core.Extensions;
using Arch.System;

namespace the_colony.Systems;

public enum Tasks
{
    Eat,
    Wander
}

public struct Eat
{
}

public struct Wander
{
}

public struct ChooseTask
{
}

public struct Task
{
    public Tasks task;
}

public partial class TaskSystem : BaseSystem<World, double>
{
    public TaskSystem(World world) : base(world)
    {
    }

    [Query]
    [All(typeof(ChooseTask))]
    private static void ChooseTask(ref Hunger hunger, Entity entity)
    {
        if (hunger.current <= 90)
            //entity.Remove<Wander>();
            entity.Add(new Eat());
        else
            //entity.Remove<Eat>();
            entity.Add(new Wander());

        //entity.Remove<ChooseTask>();
    }
}