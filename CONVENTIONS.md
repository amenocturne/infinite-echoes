# CONVENTIONS

## Project Structure

### Workspace Organization

- Use Cargo workspaces for multi-crate projects (game, server, macros)
- Organize code into logical modules with clear separation of concerns:
  - `core/` - Core game types and utilities
  - `engine/` - Game engine subsystems (audio, state management, scheduling)
  - `nodes/` - Domain-specific game objects (audio nodes, effects)
  - `render/` - Rendering and UI systems
  - `debug/` - Development and debugging tools

### Module Structure

- Each module directory must have a `mod.rs` file
- Use descriptive module names that reflect their purpose
- Keep related functionality grouped together (e.g., all widgets in `render/widgets/`)

### Frontend/Backend Separation

- Game code compiles to WASM for browser execution
- Anything not game-related should be implemented in TypeScript/JavaScript/HTML/CSS
- If a feature requires extensive frontend code, create a subfolder for the frontend part of the application

## Naming Conventions

### Code Identifiers

- **Functions and methods**: `snake_case` (e.g., `handle_client`, `update_dragged_position`)
- **Variables and fields**: `snake_case` (e.g., `file_path`, `card_size`, `audio_context`)
- **Structs and enums**: `PascalCase` (e.g., `GameEngine`, `CardType`, `NoteGenerator`)
- **Enum variants**: `PascalCase` (e.g., `HotReloading::Enabled`, `MimeType::Html`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `PORT`, `PULSES_PER_QUARTER_NOTE`)
- **Modules**: `snake_case` (e.g., `audio_engine`, `game_config`)
- **Type aliases**: `PascalCase` for types, `snake_case` for type aliases used as shortcuts

### Constructors and Factory Methods

- Use `new()` for basic constructors
- Use `with_*` for constructors with additional configuration (e.g., `with_boundary`)
- Use `from_*` for conversion constructors (e.g., `from_cards`)
- Use `default_*` for default configurations (e.g., `default_filter`)

## Error Handling

### Error Types and Results

- Use custom error types (`GameError`) with descriptive messages
- Create type aliases for common Result types: `type GameResult = Result`
- Implement helper methods for error creation:
  - `GameError::msg()` for simple string messages
  - `GameError::js()` for JavaScript/WASM error conversion

### Error Propagation

- Prefer `?` operator for error propagation
- Use `.map_err()` for error type conversion with descriptive closures
- Use `unwrap()` and `expect()` sparingly, only when failure is truly impossible
- When using `expect()`, provide meaningful error messages

### Result Handling Patterns

```rust
// Prefer this pattern for error handling
let result = risky_operation()
    .map_err(GameError::js("Failed to perform operation"))?;

// Use meaningful expect messages
let value = option_value.expect("Value should always be present at this point");
```

## Code Style and Formatting

### Indentation and Spacing

- Use 4 spaces for indentation (no tabs)
- Maximum line length: 100 characters
- Use trailing commas in multi-line function calls, struct definitions, and enums
- Separate logical sections with blank lines

### Function and Method Organization

- Keep functions focused and single-purpose
- Organize `impl` blocks by functionality (constructors first, then main methods, then utilities)
- Use descriptive parameter names that indicate purpose

### Pattern Matching

- Prefer `match` expressions over multiple `if let` statements
- Use meaningful variable names in pattern matches
- Handle all cases explicitly rather than using catch-all patterns when possible

## Type System Usage

### Custom Types

- Create domain-specific types for clarity (e.g., `GameTime`, `MusicTime`)
- Use newtypes for type safety when needed
- Implement common traits (`Clone`, `Debug`, `PartialEq`) as appropriate

### Option and Result Usage

- Use `Option` for potentially absent values
- Use `Result` for operations that can fail
- Avoid nested `Option` or `Result` types when possible

### Trait Design

- Use traits for shared behavior (e.g., `Render`, `RenderAudio`)
- Keep trait methods focused and cohesive
- Implement traits for types rather than using generic implementations when specificity is needed

## Memory Management

### Interior Mutability

- Use `RefCell` for single-threaded interior mutability
- Document why interior mutability is needed in comments
- Prefer owned data over complex borrowing when performance allows

### Collections

- Use `Vec` for dynamic arrays
- Use appropriate collection types for the use case (e.g., `HashMap` for key-value lookups)

### Performance Considerations

- Performance is not the current priority, but code should be sane
- Avoid unnecessary memory copying and slow algorithms if better alternatives exist and don't take much longer to implement
- Choose efficient data structures when the choice is obvious

## Architecture Patterns

### Component-Based Design

- Separate concerns into distinct components (audio, rendering, input handling)
- Use trait objects for polymorphic behavior when needed
- Implement event-driven patterns for loose coupling

### Configuration Management

- Use dedicated configuration structs (e.g., `GameConfig`, `DebugHudConfig`)
- Group related configuration options together
- **All magic numbers and constants should be placed in the game config struct and passed to functions that need them**
- Provide reasonable defaults

## WASM and Web Development

### Target Compatibility

- Write code that compiles for WASM target (try to depend on wasm-specific things only for audio, everything else should be using just macroquad and be compilable for native)
- Use conditional compilation (`#[cfg(target_arch = "wasm32")]`) when needed
- Prefer web-compatible APIs when available

### Build Process

- Use `wasm-bindgen` for JavaScript interop
- Organize build scripts in `justfile` for complex build processes
- Keep WASM-specific transformations documented and scripted
- **All core commands should be added to justfile**

### Audio Processing

- **Audio processing should use Web Audio API scheduling as much as possible** since the main function is async
- **Time-based audio operations must strongly prefer Web Audio API over custom implementations** for reliable synchronization
- Avoid custom timing implementations for audio-critical operations

## Documentation Standards

### Code Comments

- **Documentation should be as minimal as possible** - method names and parameters should speak for themselves
- When documentation is necessary, focus on **"Why is it so?"** rather than **"What does it do?"**
- Add small comments to confusing parts of code explaining the "why"
- Use `///` doc comments sparingly, only for public APIs that need explanation

### Comment Annotations

- Use structured comment annotations: `// KEYWORD: any related information`
- Supported keywords:
  - `TODO:` - For future improvements or features to implement
  - `NOTE:` - For important clarifications or explanations
  - `FIX:` - For known issues that need to be addressed

### Example of Good Documentation

```rust
// NOTE: RefCell needed here because of shared ownership in the audio graph
pub struct AudioGraph {
    nodes: Vec>,
}

// TODO: enhance computing using note effects
pub fn loop_length(&self) -> MusicTime {
    // Sum individual note generator lengths
    let mut len = MusicTime::ZERO;
    for n in &self.nodes {
        len = len + n.borrow().as_note_generator()
            .map(|ng| ng.loop_length)
            .unwrap_or(MusicTime::ZERO);
    }
    len
}
```

## Testing Guidelines

### Test Organization

- Use `#[cfg(test)]` modules in the same files as the code being tested[1][2]
- Place unit tests in a `tests` module within each source file
- Create integration tests in a separate `tests/` directory for cross-module testing
- Use descriptive test function names that explain what is being tested

### Test Structure

- Follow the Arrange-Act-Assert pattern
- Group related tests in nested modules when needed
- Use `#[should_panic]` for tests that expect panics with specific error messages

### Testing Patterns for This Project

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::note_generator::{NoteDuration, NoteEvent};

    #[test]
    fn audio_graph_calculates_correct_loop_length() {
        // Arrange
        let ng1 = NoteGenerator::new(NoteDuration::Quarter.into(), vec![]);
        let ng2 = NoteGenerator::new(NoteDuration::Half.into(), vec![]);
        let osc = Oscillator::new(WaveShape::Sine);

        // Act
        let graph = AudioGraph::new(vec![ng1, ng2], osc, vec![]);

        // Assert
        let expected = NoteDuration::Quarter.into() + NoteDuration::Half.into();
        assert_eq!(graph.loop_length(), expected);
    }

    #[test]
    fn card_type_converts_to_correct_audio_node_type() {
        assert_eq!(CardType::SineOscillator.as_type(), AudioNodeType::Oscillator);
        assert_eq!(CardType::NoteGenerator.as_type(), AudioNodeType::NoteGenerator);
    }
}
```

### Test-Specific Guidelines

- Test public interfaces rather than internal implementation details
- Mock external dependencies (Web Audio API calls) when possible for unit tests
- Create helper functions for common test setup patterns

## Logging and Debugging

### Development vs Production

- Use logging and debugging tools during development but keep them minimal for end users
- Avoid console output that end users would see in the browser
- Use appropriate macroquad-compatible logging functions for web targets

### Logging Levels

- Use `debug!()` for development information that should not appear in production
- Reserve `info!()` for essential user-facing information
- Use `warn!()` and `error!()` sparingly for actual issues

### Debug Tools

- Implement debug HUD systems for development builds
- Use conditional compilation for debug features: `#[cfg(debug_assertions)]`
- Keep debug information in dedicated debug modules

## Dependency Management

### Dependency Philosophy

- **Keep dependencies minimal** - only add dependencies that are very critical
- If functionality takes significant time and effort to implement and can be added as a dependency, then add it
- Otherwise, implement internally for better flexibility, compilation times, and control
- Prefer standard library solutions when available

### Dependency Guidelines

- Document why each dependency is necessary in `Cargo.toml` comments
- Use workspace dependencies for shared dependencies across crates
- Prefer dependencies with minimal transitive dependencies

### Current Dependency Rationale

- macroquad  - Essential for web graphics rendering
- wasm-bindgen - Required for WASM JavaScript interop (used to combine both macroquad and web-sys)
- web-sys - Needed for Web Audio API bindings (mainly for working with audio as macroquad does not provide web api)
