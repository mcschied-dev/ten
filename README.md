# Space Invaders Game in Rust

A simple Space Invaders clone built using the [ggez](https://ggez.rs/) game development library. This game features dynamic gameplay with increasing difficulty and an engaging background text scroll.

## Features

- **Dynamic Gameplay**: 
  - The game becomes progressively harder as enemies move faster with each wave.
  - The player's base station grows wider with more shots per wave.
- **Scrolling Background**: 
  - A continuous text scroll ("Happy New Year Octavio") provides a parallax effect.
- **Enemy Waves**:
  - Each wave adds more enemies to the game.
- **High Scores**:
  - Earn 10 points for each enemy destroyed, with a live score displayed.
- **Game Over Screen**:
  - If enemies reach the defender line, the game ends and displays a "Game Over" message.
- **Restart Option**:
  - Press `R` to reset the game and start over.

## Controls

- **Move Left**: Press `Left Arrow`
- **Move Right**: Press `Right Arrow`
- **Shoot**: Press `Space`
- **Restart Game**: Press `R` (when in Game Over state)

## Requirements

- **Rust**: Ensure you have the Rust toolchain installed. If not, download it from [rust-lang.org](https://www.rust-lang.org/).
- **ggez**: The game engine used for development. It's included as a dependency in the `Cargo.toml`.

## Setup

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/space-invaders.git
   cd space-invaders
