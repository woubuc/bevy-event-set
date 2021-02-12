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
//!
//! // Add the event set to your app
//! App::build()
//!     .add_event_set::<MyEvents>();
//! ```

use bevy::app::AppBuilder;

/// Describes an event set
pub trait EventSet {
	fn apply(app: &mut AppBuilder);
}

/// Trait used to add `add_event_set` to the Bevy app builder
pub trait AddEventSet {
	/// Adds an event set to the app
	///
	/// # Example
	/// ```
	/// use bevy::prelude::*;
	/// use bevy_event_set::*;
	///
	/// struct MyEvent;
	/// event_set!(MyEventSet { MyEvent });
	///
	/// App::build().add_event_set::<MyEventSet>();
	/// ```
	fn add_event_set<E: EventSet>(&mut self) -> &mut Self;
}

impl AddEventSet for AppBuilder {
	fn add_event_set<E: EventSet>(&mut self) -> &mut Self {
		E::apply(self);
		self
	}
}

/// Allows an event set to send an event of a given type
pub trait SendEvent<T> {
	/// Sends an event to the event buffer
	///
	/// Calls [`Events.send`](bevy::app::Events::send()) on the Bevy event buffer of the corresponding type.
	fn send(&mut self, event: T);
}

/// Creates an event set
///
/// See the [crate-level documentation](./index.html) to see how to use this macro.
#[macro_export]
macro_rules! event_set {
	($name:ident {}) => {
		compile_error!("cannot make an empty event set");
	};
	($name:ident { $($event:ident),* $(,)? }) => {
		#[allow(non_snake_case)]
		#[derive(bevy::ecs::SystemParam)]
		pub struct $name<'a> {
			$(
				$event: bevy::ecs::ResMut<'a, bevy::app::Events<$event>>,
			)*
		}

		impl<'a> $crate::EventSet for $name<'a> {
			fn apply(app: &mut bevy::app::AppBuilder) {
				$(
					app.add_event::<$event>();
				)*
			}
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

#[cfg(test)]
mod tests {
	// These tests just check if the macros compile

	use super::*;

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
		}

		event_set!(MyEvents {
			TestEvent1,
			TestEvent2
		});
	}

	#[test]
	fn add_to_builder() {
		use bevy::app::App;

		struct TestEvent;
		event_set!(MyEvents { TestEvent });

		App::build().add_event_set::<MyEvents>();
	}
}
