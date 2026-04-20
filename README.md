# Quartz project layout

A reference for how to structure a Rust project that uses the Quartz game engine.

---

## Directory structure

```
my-quartz-game/
├── Cargo.toml
├── design.txt
├── resources/
│   ├── triangle.png
│   └── circle.png
└── src/
    ├── lib.rs
    ├── constants.rs
    ├── preferences.rs
    ├── objects/
    │   ├── mod.rs
    │   ├── triangle_obj.rs
    │   └── circle_obj.rs
    └── logic/
        ├── mod.rs
        ├── triangle_obj.rs
        └── circle_obj.rs
```

This is a library-only crate. There is no `main.rs` and no `App` struct — `ramp::run!` in `lib.rs` is the entry point. Object files in `objects/` typically have a matching file in `logic/`, but `logic/` can also contain standalone files with no corresponding object.

---

## `design.txt`

Written before any code. Describes the game concept, every object, every canvas variable, and the expected behaviour of each file. Use it as a living reference while building — update it as the design changes.

A `design.txt` for the example project above would look like this:

```
================================================================================
  CONCEPT
================================================================================

Two shapes interact using physics. Triangle is controlled by the player.
Circle bounces on collision and plays a sound.

================================================================================
  CANVAS VARS
================================================================================

score  : u32   — increments each time circle collides with triangle
active : bool  — whether the game loop is accepting input

================================================================================
  CONSTANTS  (src/constants.rs)
================================================================================

GRAVITY     f32   Downward acceleration applied to both objects each tick.
MOVE_SPEED  f32   Horizontal momentum applied to triangle on left/right input.
JUMP_FORCE  f32   Negative vertical momentum applied to triangle on space.

LAYER_BG    i32   Render layer for background objects.
LAYER_WORLD i32   Render layer for gameplay objects (triangle, circle).
LAYER_HUD   i32   Render layer for UI elements drawn on top of everything.

================================================================================
  PREFERENCES  (src/preferences.rs)
================================================================================

Holds runtime-adjustable settings that survive recompile. Distinct from
constants — these can be loaded from a config file or changed by the user.

master_volume  f32   Overall volume multiplier. Default 1.0.
music_volume   f32   Music channel multiplier. Default 0.8.
sfx_volume     f32   Sound effects channel multiplier. Default 1.0.

================================================================================
  OBJECTS  (src/objects/)
================================================================================

Each file in objects/ has a single pub fn setup(cv: &mut Canvas).
It constructs GameObjects using the builder and adds them to the canvas.
No event handlers. No on_update. Construction only.

--------------------------------------------------------------------------------
  objects/triangle_obj.rs
--------------------------------------------------------------------------------

Builds the triangle object and adds it to the canvas.

  GameObject::build("triangle")
    id        "triangle"   — unique name used to look up this object
    position  (300, 200)   — starting position in virtual canvas space
    size      (80, 80)     — width and height in pixels
    tag       "triangle"   — tag used by logic to target this object
    solid()                — enables rectangular collision
    gravity   GRAVITY      — applies downward acceleration each tick
    layer     LAYER_WORLD  — drawn on the world render layer
    image     triangle.png — sprite loaded from resources/

--------------------------------------------------------------------------------
  objects/circle_obj.rs
--------------------------------------------------------------------------------

Builds the circle object and adds it to the canvas.

  GameObject::build("circle")
    id        "circle"     — unique name
    position  (500, 200)   — starting position
    size      (64, 64)     — width and height
    tag       "circle"     — tag used by logic
    solid_circle(32.0)     — enables circular collision with radius 32
    gravity   GRAVITY      — same downward pull as triangle
    bouncy()               — sets elasticity so circle rebounds on impact
    layer     LAYER_WORLD  — same render layer as triangle
    image     circle.png   — sprite loaded from resources/

================================================================================
  LOGIC  (src/logic/)
================================================================================

Each file in logic/ has a single pub fn register(cv: &mut Canvas).
It registers on_update callbacks and GameEvent handlers for its object.
No GameObject construction. No builder calls. Behaviour only.

--------------------------------------------------------------------------------
  logic/triangle_obj.rs
--------------------------------------------------------------------------------

Handles player input and attaches a collision event to the triangle.

  on_update
    key "left"   → apply_momentum("triangle", -MOVE_SPEED, 0)   move left
    key "right"  → apply_momentum("triangle",  MOVE_SPEED, 0)   move right
    key "space"  → apply_momentum("triangle", 0, JUMP_FORCE)    jump

  add_event  GameEvent::Collision  Target::tag("triangle")
    Fires whenever triangle overlaps a solid object.

--------------------------------------------------------------------------------
  logic/circle_obj.rs
--------------------------------------------------------------------------------

Detects collision between circle and triangle, plays a sound, increments score.

  on_update
    collision_between("circle", "triangle")
      → play_sound("resources/sounds/bounce.ogg")
      → mod_var("score", Add, 1)

  add_event  GameEvent::Collision  Target::tag("circle")
    Fires whenever circle overlaps a solid object.

================================================================================
  LIB  (src/lib.rs)
================================================================================

Module declarations at the top. ramp::run! at the bottom.
No App struct. No main.rs. This is a library-only crate.

  Scene::new  CanvasMode::Landscape   16:9 virtual resolution (3840x2160)

  setup order
    objects::triangle_obj::setup   add triangle to canvas
    objects::circle_obj::setup     add circle to canvas

  register order
    logic::triangle_obj::register  attach triangle input + collision event
    logic::circle_obj::register    attach circle collision + sound logic

  ramp::run! passes context and assets in and starts the loop.

================================================================================
  RESOURCES  (resources/)
================================================================================

All files loaded at runtime by path string. Never embedded in source.

  triangle.png   sprite for the triangle object
  circle.png     sprite for the circle object
  sounds/
    bounce.ogg   played when circle collides with triangle
    music.ogg    background music (started in a scene)
  fonts/
    JetBrainsMono-Regular.ttf   used for any HUD text objects
```

---

## `Cargo.toml`

```toml
[package]
name    = "my-game"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
ramp   = { package = "ramp2", path = "../ramp" }
quartz = { path = "../quartz" }
```

---

## `resources/`

All runtime assets — images, fonts, and audio files. Loaded at runtime via `load_image()`, `font()`, and `Canvas::play_sound()`. Subdirectories are optional but recommended for larger projects.

```
resources/
  images/
    triangle.png
    circle.png
  fonts/
    JetBrainsMono-Regular.ttf
  sounds/
    bounce.ogg
    music.ogg
```

---

## `src/`

### `lib.rs`

Module declarations at the top, then `ramp::run!` at the bottom. No `App` struct, no `main.rs`. This is a library-only crate — `ramp::run!` is the entry point.

```rust
pub mod constants;
pub mod preferences;
pub mod objects;
pub mod logic;

use ramp::prelude::*;

ramp::run! { |context: &mut Context, assets: Assets| {
    let mut scene = Scene::new(context, CanvasMode::Landscape, 1);
    let layer_id  = LayerId(0);

    let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();

    // construct objects
    objects::triangle_obj::setup(cv);
    objects::circle_obj::setup(cv);

    // register logic
    logic::triangle_obj::register(cv);
    logic::circle_obj::register(cv);

    scene
}}
```

### `constants.rs`

Game-wide compile-time constants — speeds, sizes, layer IDs, forces. Import with `use crate::constants::*`.

```rust
pub const GRAVITY:      f32 = 1800.0;
pub const MOVE_SPEED:   f32 = 420.0;
pub const JUMP_FORCE:   f32 = -900.0;

pub const LAYER_BG:     i32 = 0;
pub const LAYER_WORLD:  i32 = 1;
pub const LAYER_HUD:    i32 = 10;
```

### `preferences.rs`

Runtime-adjustable settings — volume, difficulty, control bindings. Separate from `constants.rs` so user config never touches compiled constants. Can be serialised to disk if needed.

```rust
pub struct Preferences {
    pub master_volume: f32,
    pub music_volume:  f32,
    pub sfx_volume:    f32,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume:  0.8,
            sfx_volume:    1.0,
        }
    }
}
```

---

## `src/objects/`

**Scene construction only.** Each file builds `GameObject`s and adds them to the canvas. No event handlers, no per-tick logic.

### `objects/mod.rs`

```rust
pub mod triangle_obj;
pub mod circle_obj;
```

### `objects/triangle_obj.rs`

```rust
use ramp::prelude::*;
use crate::constants::*;

pub fn setup(cv: &mut Canvas) {
    let obj = GameObject::build("triangle")
        .position(300.0, 200.0)
        .size(80.0, 80.0)
        .tag("triangle")
        .solid()
        .gravity(GRAVITY)
        .layer(LAYER_WORLD)
        .image(load_image("resources/triangle.png"))
        .finish();

    cv.add_game_object("triangle".into(), obj);
}
```

### `objects/circle_obj.rs`

```rust
use ramp::prelude::*;
use crate::constants::*;

pub fn setup(cv: &mut Canvas) {
    let obj = GameObject::build("circle")
        .position(500.0, 200.0)
        .size(64.0, 64.0)
        .tag("circle")
        .solid_circle(32.0)
        .gravity(GRAVITY)
        .bouncy()
        .layer(LAYER_WORLD)
        .image(load_image("resources/circle.png"))
        .finish();

    cv.add_game_object("circle".into(), obj);
}
```

---

## `src/logic/`

**Behaviour only.** Each file registers `on_update` callbacks and `GameEvent` handlers. No object construction here. Most logic files correspond to an object file in `objects/`, but logic files can also be standalone — for things like input managers, game state machines, score systems, or any behaviour that isn't tied to a single object.

### `logic/mod.rs`

```rust
pub mod triangle_obj;
pub mod circle_obj;
```

### `logic/triangle_obj.rs`

```rust
use ramp::prelude::*;
use crate::constants::*;

pub fn register(cv: &mut Canvas) {
    cv.on_update(|c| {
        if c.key("left") {
            c.run(Action::apply_momentum(
                Target::tag("triangle"), -MOVE_SPEED, 0.0,
            ));
        }
        if c.key("right") {
            c.run(Action::apply_momentum(
                Target::tag("triangle"), MOVE_SPEED, 0.0,
            ));
        }
        if c.key("space") {
            c.run(Action::apply_momentum(
                Target::tag("triangle"), 0.0, JUMP_FORCE,
            ));
        }
    });

    cv.add_event(GameEvent::Collision, Target::tag("triangle"));
}
```

### `logic/circle_obj.rs`

```rust
use ramp::prelude::*;

pub fn register(cv: &mut Canvas) {
    cv.add_event(GameEvent::Collision, Target::tag("circle"));

    cv.on_update(|c| {
        if c.collision_between(&Target::tag("circle"), &Target::tag("triangle")) {
            c.run(Action::play_sound("resources/sounds/bounce.ogg"));
        }
    });
}
```

---

## Rules

- `objects/` constructs — `logic/` behaves. Never mix the two.
- Most object files have a matching logic file, but logic files do not need a corresponding object. Standalone logic files are fine for things like input managers, score systems, or game state machines.
- `constants.rs` is for compile-time values. `preferences.rs` is for runtime/user-adjustable values.
- `lib.rs` is wiring only. If logic is creeping in, it belongs in `logic/`.
- There is no `main.rs`. There is no `App` struct. `ramp::run!` is the entry point.
- Assets live in `resources/`. Load them by path string at runtime — never embed binary assets in source files.
- Write `design.txt` first. Every object, variable, and behaviour should be described there before the matching code is written.
