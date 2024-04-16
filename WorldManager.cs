using System.Diagnostics;
using Arch.Core;
using Arch.System;
using Godot;
using the_colony.Systems;

public partial class WorldManager : Node
{
    public static World world;
    public static AStar2D aStar = new();
    private Group<double> _systems;

    private TileMap tileMap;

    public override void _Ready()
    {
        tileMap = GetNode<TileMap>("/root/root/TileMap");
        // Create a world and a group of systems which will be controlled
        world = World.Create();
        _systems = new Group<double>(
            "Main",
            new MovementSystem(world), // Run in order
            new HungerSystem(world),
            new TaskSystem(world)
            //new MyOtherSystem(...),
        );
        _systems.Initialize(); // Inits all registered systems

        updateAStar();
    }

    // Called every frame. 'delta' is the elapsed time since the previous frame.
    public override void _Process(double delta)
    {
        _systems.BeforeUpdate(in delta); // Calls .BeforeUpdate on all systems ( can be overriden )
        _systems.Update(in delta); // Calls .Update on all systems ( can be overriden )
        _systems.AfterUpdate(in delta); // Calls .AfterUpdate on all System ( can be overriden )
        //_systems.Dispose(); // Calls .Dispose on all systems ( can be overriden )
    }

    public long getCellId(Vector2 tile, Rect2I rect)
    {
        var id = (long)(tile.X - rect.Position.X + (tile.Y - rect.Position.Y) * rect.Size.X);

        return id;
    }

    public void updateAStar()
    {
        var tiles = tileMap.GetUsedCells(0);
        var rect = tileMap.GetUsedRect();

        var offset = new Vector2(rect.Position.X, rect.Position.Y);
        offset *= 16;
        Debug.Print($"TileMap offset: {offset}");

        foreach (var tile in tiles)
        {
            var tileData = tileMap.GetCellTileData(0, tile);

            // TODO: This seems to be returning the wrong data in some cases, breaking everything
            tileData.SetCustomData("astarId", getCellId(tile, rect));

            var cost = tileData.GetCustomData("MovementCost").As<float>();
            aStar.AddPoint(getCellId(tile, rect), tileMap.MapToLocal(tile), cost);
        }

        foreach (var tile in tiles)
        foreach (var neighbor in tileMap.GetSurroundingCells(tile))
        {
            var neighborTileData = tileMap.GetCellTileData(0, neighbor);

            if (neighborTileData == null) continue;

            // TODO: This seems to be returning the wrong data in some cases, breaking everything
            var neighborId = neighborTileData.GetCustomData("astarId").As<long>();

            aStar.ConnectPoints(getCellId(tile, rect), getCellId(neighbor, rect));
        }
    }
}