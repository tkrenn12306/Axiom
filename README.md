# Axiom

> *"From first principles, simulate everything."*

**Axiom** is an open-source, terminal-based open-world physics simulation framework written in Rust.

It brings its own declarative language (**AxiomLang**, file extension `.ax`) for defining worlds, terrain, weather, entities, and physical laws — all rendered in your terminal via TUI.

## Features

- **No GUI required** — everything runs in the terminal (ASCII/Unicode rendering + TUI)
- **Real physics** — Newtonian mechanics, thermodynamics, fluid dynamics, material physics
- **Declarative configuration** — worlds defined in `.ax` files, fully shareable
- **Composable modules** — every physical system is a module, toggleable
- **Headless-capable** — simulations run without rendering (for AI training, batch simulation, tests)

## Quick Start

```bash
# Run the interactive REPL
cargo run -p axiom-cli

# Load a scenario
axiom load scenarios/test_basic.ax

# Inspect an entity
axiom inspect entity.Elena
```

## Project Structure

```
crates/
  axiom-core/     ECS engine, tick loop, event bus
  axiom-lang/     AxiomLang parser and evaluator
  axiom-physics/  Physics systems (mechanics, thermo, fluids, atmosphere)
  axiom-ai/       Behavior trees and pathfinding
  axiom-render/   Terminal rendering (ratatui)
  axiom-cli/      CLI and REPL
stdlib/           Standard library .ax definitions
scenarios/        Example simulations
```

## Status

Pre-Alpha — Phase 0 (Foundation) in progress.

## License

MIT
