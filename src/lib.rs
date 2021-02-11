//! A macro to create event bundles for Bevy
//!
//! # Usage
//! ```ignore
//! event_set!([name] { [...events] });
//! ```
//!
//! This will create a struct that can be used to send events of all given
//! types through the [`SendEvent`] trait.
//!
//! # Example
//! ```
//! # use bevy::prelude::*;
//! use bevy_event_set::*;
//!
//! // Define your events
//! struct EventOne;
//! struct EventTwo;
//! struct EventThree(usize);
//!
//! // Create an event set named `MyEvents`
//! event_set!(MyEvents { EventOne, EventTwo, EventThree });
//!
//! // Use the event set in a system
//! fn event_emitter_system(mut events: MyEvents) {
//!     events.send(EventOne);
//!     events.send(EventTwo);
//!     events.send(EventThree(42));
//! }
//!
//! // Subscribe to events selectively in different systems
//! fn event_one_listener_system(events: Res<Events<EventOne>>) { }
//! fn event_two_listener_system(events: Res<Events<EventTwo>>) { }
//! fn event_three_listener_system(events: Res<Events<EventThree>>) { }
//! ```

/// Creates an event set
///
/// See the [crate-level documentation](./index.html) to see how to use this macro.
#[macro_export]
macro_rules! event_set {
	($name:ident { $($event:ident),* $(,)? }) => {
		#[allow(non_snake_case)]
		#[derive(bevy::ecs::SystemParam)]
		pub struct $name<'a> {
			$(
				$event: bevy::ecs::ResMut<'a, bevy::app::Events<$event>>,
			)*
		}

		$(
			impl<'a> $crate::SendEvent<$event> for $name<'a> {
				fn send(&mut self, event: $event) {
					self.$event.send(event)
				}
			}
		)*
	};
}

/// Allows an event set to send an event of a given type
pub trait SendEvent<T> {
	/// Sends an event to the event buffer
	///
	/// Calls [`Events.send`](bevy::app::Events::send()) on the Bevy event buffer of the corresponding type.
	fn send(&mut self, event: T);
}

#[cfg(test)]
mod tests {
	// These tests just check if the macros compile

	#[test]
	fn single() {
		struct TestEvent;
		event_set!(MyEvents { TestEvent });
		event_set!(MyEvents2 { TestEvent });
	}

	#[test]
	fn multiple() {
		struct TestEvent1(usize);
		struct TestEvent2 {
			number: usize,
		};

		event_set!(MyEvents {
			TestEvent1,
			TestEvent2
		});
	}
}
