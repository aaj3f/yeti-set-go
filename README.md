# Yeti, Set, Go!

A fast-paced skiing game built with Rust and macroquad. Guide your yeti down the mountain, collect items, and avoid obstacles while climbing the leaderboards.

## Features

- **Multi-level progression** with increasing difficulty
- **Real-time scoring system** with level-based multipliers
- **Remote leaderboards** powered by Fluree database
- **Responsive design** with custom typography and theming
- **Dev mode** for rapid UI development and testing

## Prerequisites

- Rust 1.70+ (with Cargo)
- FLUREE_API_KEY environment variable (for leaderboard functionality)

## Getting Started

### Installation

```bash
git clone https://github.com/aaj3f/yeti-set-go
cd yeti-set-go
```

### Running the Game

```bash
# Set your Fluree API key (required for leaderboards)
export FLUREE_API_KEY="your_fluree_api_key_here"

# Run the game
cargo run
```

### Building for Release

```bash
cargo build --profile dist
```

## Game Controls

- **Arrow Keys / WASD**: Move the yeti
- **SPACE**: Confirm selections / Return to menu
- **ESC**: Return to menu / Exit
- **D**: Toggle dev mode (if enabled)

## Environment Variables

- `FLUREE_API_KEY`: Required for remote leaderboard functionality. Without this, the game runs in offline mode with local scores only.

## Architecture

The game is built with a modular architecture:

- **Game Engine**: Built on macroquad for cross-platform 2D graphics
- **State Management**: Clean separation of game states (Menu, Playing, GameOver, etc.)
- **API Integration**: Async background networking with local fallback
- **Design System**: Semantic typography and color theming
- **Asset Management**: Centralized loading and caching

## Development

### Dev Mode

Enable dev mode in `src/config.rs` and press `D` during gameplay to access:

- UI component previews
- Typography showcase
- Color palette viewer
- Mock data testing
- Overlay toggle (H key)

### Project Structure

```
src/
├── main.rs          # Entry point and game loop
├── game/            # Core game logic and state management
├── entities/        # Game objects (Yeti, Items)
├── ui/              # User interface components
├── api.rs           # Fluree database integration
├── design.rs        # Typography and theming system
├── assets.rs        # Asset loading and management
└── config.rs        # Game configuration
```

## Contributing

This project uses a semantic design system and follows Rust best practices. When contributing:

1. Follow existing code patterns and naming conventions
2. Use the design system for UI consistency
3. Test with dev mode for UI changes
4. Ensure API fallback behavior works correctly

## License

MIT License - see LICENSE file for details.