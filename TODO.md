# To Do

## Redesign ideas
[ ] Instead of having a generic Action for all bindings, I should just have one struct for all bindings...
        because I cannot have mutable access in multiple systems at the same time, so accessing Action
        events can only be done by one system at a time... which defeats the purpose of this ECS, dependency
        injection-like system.

## Input mechanics
[ ] Promote inputs to configuration file: read on startup

## FPS Spectator Camera mechanics
[x] WASD movement
[ ] Space (jump), C (crouch) for up and down movement
[x] Mouse axes to look around
    [ ] Clamp the pitch so that you can't flip upside down

## RTS Spectator Camera mechanics
[ ] Mouse cursor on edge of screen to pan the camera
[ ] Mouse wheel to zoom in and out (clamped)
[ ] Arrow keys (instead of WASD) movement

## Command Console mechanics
[ ] Press ~ to toggle the Console
[ ] Log messages to Console (ideally via `utils::logging::Logger`)
[ ] Use Text Input in the Console
[ ] Command Parsing (e.g. spawn a cube via console command)

## Physics
[ ] Implement collision volumes so that I can react to "overlap" events
[ ] Figure out the "proper" way to do movement with Heron (rotational force)

## UI mechanics
[ ] Main menu
[ ] Options menu
[ ] Hotkey bindings configuration, via Settings menu

# AI
[ ] Pathfinding: to navigate and move along a path