# bevy_event_set
A macro to create event bundles for Bevy

Standard practice in Bevy currently is declaring events as an enum. This is
fine for many use cases, but in some situations you want to be able to listen
for individual events in your systems while still being able to easily send
multiple types of events (e.g. when parsing user input).

With the `event_set` macro, you can create an event set that allows you to send
multiple event types.

## Setup
A bug in a subcrate of Bevy 0.4 prevents this crate from working properly. Add
the following patch to your Cargo.toml to apply the fix:

```toml
[patch.crates-io]
bevy_ecs_macros = { git = "https://github.com/woubuc/bevy", branch = "fix/ecs-macro-systemparam-0.4" }
```

This bug is fixed in Bevy with PR [#1434](https://github.com/bevyengine/bevy/pull/1434).


## Usage
```rust
use bevy::prelude::*;
use bevy_event_set::*;

// Define your events
struct EventOne;
struct EventTwo;
struct EventThree(usize);

// Create an event set named `MyEvents`
event_set!(MyEvents { EventOne, EventTwo, EventThree });

// Use the event set in a system
fn event_emitter_system(mut events: MyEvents) {
    events.send(EventOne);
    events.send(EventTwo);
    events.send(EventThree(42));
}

// Subscribe to events selectively in different systems
fn event_one_listener_system(events: Res<Events<EventOne>>) { }
fn event_two_listener_system(events: Res<Events<EventTwo>>) { }
fn event_three_listener_system(events: Res<Events<EventThree>>) { }
```
