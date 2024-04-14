using Arch.Core;
using Arch.System;
using Godot;

public partial class WorldManager : Node
{
    public static World world;
    private Group<double> _systems;

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        // Create a world and a group of systems which will be controlled
        world = World.Create();
        _systems = new Group<double>(
            "Main",
            new MovementSystem(world) // Run in order
            //new MyOtherSystem(...),
        );
        _systems.Initialize(); // Inits all registered systems
    }

    // Called every frame. 'delta' is the elapsed time since the previous frame.
    public override void _Process(double delta)
    {
        _systems.BeforeUpdate(in delta); // Calls .BeforeUpdate on all systems ( can be overriden )
        _systems.Update(in delta); // Calls .Update on all systems ( can be overriden )
        _systems.AfterUpdate(in delta); // Calls .AfterUpdate on all System ( can be overriden )
        //_systems.Dispose(); // Calls .Dispose on all systems ( can be overriden )
    }
}