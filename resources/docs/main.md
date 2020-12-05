# Main method
- Gather config file for game, window, graphics info
- Create ggez context from window/graphics info
- Initialize ./resources path for ggez (from cargo manifest dir)
- Build ggez event loop
- Initialize game state with ggez context and window info
- Load level from config's start level name
- Begin running event loop

## Modules used
- conf - to get game config
- game_state - create new game state, load level
