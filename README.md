# fooling-around

A first-person action RPG prototype built in
[Godot 4](https://godotengine.org/), with gameplay logic in Rust via
[godot-rust (gdext)](https://godot-rust.github.io/).

> Working title. A learning project and a testbed for the design goals below —
> not a finished game.

## Vision

A first-person action RPG built around:

- **Physically grounded movement** — momentum, weight, and consequence over
  arcade responsiveness
- **Tactical, deliberate combat** — positioning and preparation matter more than
  inflated stats or reflex-mashing
- **Dynamic combat outcomes** beyond HP depletion: stagger, knockback,
  knockdown, disarm, wounds, interruption, morale, forced retreats, terrain
  advantage, weapon switching
- **A believable world** — geography, history, and population thought through
  before story or player-facing content
- **Meaningful exploration** — peaceful, atmospheric, driven by ambient wildlife
  and occasional, non-scripted encounters (patrols, caravans, travelling
  merchants)
- **Emergent AI** — enemies and NPCs that make situational decisions rather than
  follow scripted behavior trees

## Design philosophy

- Build systems and prototypes before content or art.
- Every experiment should produce something observable and playable.
- No procedural generation without an immediate gameplay purpose.
- Primitives and placeholders over spending time on art.
- Clear separation of concerns: input → intent → physics → AI decisions → combat
  consequences.
- Prefer explicit, readable code over engine magic or dense abstraction.
- A brand-new scene or mechanic may get a quick GDScript version first, just to
  feel it out — but anything that stays gets ported to Rust. Shipped gameplay
  code is Rust-only: static typing catches a class of bugs GDScript's dynamic
  typing lets through at runtime, and it's testable in isolation (`cargo test`,
  independent of a running Godot instance) — both of which matter for the
  systemic depth (combat math, AI decisions, save-state integrity) this project
  is aiming for.

## Current status

Basics work: first-person player moves, looks around, jumps, collides with the
world. One NPC exists as a patrol test case.

- **Player** — WASD, mouse look, jump, gravity, collision.
- **NPC** — patrols by picking a random point inside a bounding area, walking to
  it, then pausing for a random duration before picking the next point; turns to
  face its movement target at a capped turn speed, independent of translation.
  Runs on the same movement code as the player.
- **Movement** — acceleration/deceleration is currently _disabled_: velocity
  snaps straight to the target each frame instead of ramping. Ramped
  acceleration was tried and shelved — hard to judge "correct" values in an
  empty, textureless test scene, it just felt like walking on ice. Revisit once
  there's real level geometry to tune against.

Known open issues, not yet fixed:

- The (currently unused) acceleration step size isn't scaled by frame delta —
  will be framerate-dependent once ramping comes back.
- Gravity and horizontal acceleration are applied in a fixed order in the shared
  movement code; swapping it would make the two fight each other. Not documented
  in-code yet.

## Roadmap

Roughly the order things are being tackled — not a rigid plan, adjusted as
prototyping reveals what's actually fun/necessary.

- [x] Minimal 3D staging ground (flat floor, basic lighting, no unnecessary
      assets)
- [x] First-person player controller (movement, look, jump, gravity, collision)
- [x] First dumb NPC (patrol behavior, shared movement pipeline)
- [ ] Combat prototype — sword and bow, one dummy enemy, then multiple/varied
      encounters
- [ ] NPC detection (player awareness, line-of-sight)
- [ ] NPC chase behavior
- [ ] Free/libre placeholder assets (only to de-abstract prototypes, not final
      art)
- [ ] Physics experiments: knockback, terrain interaction, physical consequences
- [ ] Re-introduce acceleration/deceleration, tuned against real level geometry
- [ ] Needs/opportunity-based NPC AI (hunger, safety, social, sleep; scored
      action selection)
- [ ] Procedural generation — only once it serves a concrete gameplay purpose
      (e.g. combat arenas), not for world generation up front

## Combat design notes

Not yet implemented — captured here as guardrails for when combat prototyping
begins:

- Simple controls, complicated consequences
- No Souls-style dodge-pattern memorization, no Skyrim-style hack-and-slash
- Positioning and timing matter more than stamina-bar management
- Directional attacks/defenses should be forgiving, not pixel-perfect
- Weapon choice and preparation should meaningfully change difficulty
- Enemies are not just HP bars — engagements should visibly shift with stagger,
  morale, and battlefield state

## Development environment

This repo assumes a Nix-based dev environment (`flake.nix` provides `cargo`,
`rustc`, `rust-analyzer`, `godot_4`, and supporting build tools). Run
`direnv allow`, or enter the flake's devShell manually.

```
fooling-around/
├── godot/     # Godot project (project.godot, scenes, assets)
└── rust/      # gdext crate (Cargo workspace)
```

---

_This is a personal learning project. Contributions, issues, and unsolicited
opinions about combat design are all welcome anyway._
