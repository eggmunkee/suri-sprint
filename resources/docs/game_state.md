# Game State module
Game state combines graphics event loop with specs game information and 
physics data. It interacts with audio, resources, components, systems, 
world, entities, physics, render, and input modules.

## States/Modes
### RunningState - status of game itself
- Playing - normal playing state - run world & physics
- Dialog(String) - dialog being shown - paused world

### State - status of game interface
- Running - not paused
- Paused - paused

### GameMode - whether edit level or regular mode
- Play
- Edit

## GameState struct
Game state data and references including window_title, state, runningState, 
level, window w/h, world, font, physicsWorld, displayScale, pausedText, 
currentViewOffset, clickInfo, levelWarping, levelWarpTimer, warpLevelName,
warpLevelEntryName, pausedAnim, and audio.

### new() method
New Game State created with ggez context, window title, and window mode.

Create physics world and game state resource. Create ggez window from 
context. Create font and paused text. Create specs world with ggez 
context, game state resource, and physics world. Finally create game state.

### GameState interal implementation / methods
#### Soundsystem
- play_music
- dim_music
#### Play/Pause state - update audio also
- pause
- play
#### Time tracking
- reset_level_time - sets world time in GateStateResource
- set_frame_time - sets world frame time and increment world total time
- update_run_time - increments program run time - outside of game world time
- set_running_state - set running state (playing/dialog) - update audio
#### Game World Update
- run_update_systems
    - logic system
    - input system - create meows if needed, check click info
    - meow system - advance and self-delete
    - portal system - update portals
    - button system - handle buttons directed at spawn methods (test)    
- run_dialog_update - just run input system
- run_post_physics_update
    - Handle activity with characters, npcs, and portals
    - Character exit handling - save exit start info
    - General portal handling - update collider component pos/velocity
    - Save facing right flag to char/npc
    - Init start_warp if char newly entered exit
- process_time_delta
    - handle level warp timer, return frame time_delta
- run_update_step
    - update_run_time, process_time_delta, set_frame_time
    - switch by running_state:
        - Playing - run_update_systems then do "maintain" step for world, advance physics,
         run_post_physics_update.
        - Dialog - run handle_dialog_input and determine if running_state changed
    - update running state if changed
- clear_world - delete all content from world
- clear_physics - create new empty physics world
- set_level_bounds - set level bounds in GameStateResource
- start_warp - set new warp status and restart warp timer
- save_level - save current level to a given save name within levels
- actual_load_level
    - load level config from file
    - dim audio, and start new music if needed
    - set_level_bounds
    - build level from level config
    - reset_level_time
- load_level
    - set level name and entry name
    - set dialog state with level info
    - clear world, physics
    - actual_load_level
- game_key_down - handle game key events
    - Exit key - quit
    - Pause key - toggle paused state (except within dialogs)
- game_key_up

### GameState ggez event::EventHandler implementation
- update
    - perform update step of game loop
    - if running, call run_update_step
    - paused, increment paused animation
- draw
    - create renderer
    - render frame
    - set current display offset
    - yield to OS    
#### InputMap processed events:
- mouse events (button down/up, motion, wheel) - all to InputMap
- key down event
    - handle zoom & volume controls directly held down, set display scale/volume
    - if InputMap passes back a game key, pass to game_key_down
- key up event
    - J - generate a platform, box, or ghost (test method)
    - subtract/equals - display scale
    - F1 - toggle edit/play mode
    - R - reload level
    - [ or ] - volume
    - if InputMap passes back a game key, pass to game_key_up
- gamepad events (button down/up, axis)
    - pass to InputMap, pass game keys to game_key_xx    
- text_input_event
- focus_event
- resize_event - update GameStateResource to new width/height, update ggez coords

### GameStateClickInfo
Struct to respond to report_fixture() from physics queries and save 
results. Rsults are pushed into click_info.
