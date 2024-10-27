// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

//! Traits for constructing/destructuring from/into a type's internal guts.
//!
//! # Example
//!
//! ```
//! mod state_machine {
//!     use guts::{Guts, FromGutsUnchecked};
//!
//!     /// A State machine's internal state.
//!     pub enum State {
//!         Off,
//!         On,
//!     }
//!
//!     /// A State machine that hides its internal state.
//!     pub struct StateMachine {
//!         state: State,
//!     }
//!
//!     impl Default for StateMachine {
//!         /// Creates a state machine in the only allowed initial state: `Off`.
//!         fn default() -> Self {
//!             Self { state: State::Off }
//!         }
//!     }
//!
//!     impl Guts for StateMachine {
//!         type Guts = State;
//!     }
//!
//!     impl FromGutsUnchecked for StateMachine {
//!         /// Creates a state machine in an arbitrary state, unsafely.
//!         unsafe fn from_guts_unchecked(guts: Self::Guts) -> Self {
//!             Self { state: guts }
//!         }
//!     }
//! }
//!
//! use guts::FromGutsUnchecked;
//! use state_machine::{State, StateMachine};
//!
//! // A machine can easily be safely created in its initial state:
//! let machine = StateMachine::default();
//!
//! // To create a machine in a non-initial state `unsafe { … }` is required:
//! let machine = unsafe { StateMachine::from_guts_unchecked(State::On) };
//! ```

#![cfg_attr(feature = "never_type", feature(never_type))]

/// The base trait of `FromGuts` and `IntoGuts`, its more useful companions.
pub trait Guts: Sized {
    /// The type's guts.
    type Guts;
}

/// Safely destructuring values into their guts.
pub trait IntoGuts: Guts {
    /// Destructures a value into its guts.
    fn into_guts(self) -> Self::Guts;
}

/// Safely constructing values from their guts.
pub trait FromGuts: Guts {
    /// Constructs a value from its guts.
    fn from_guts(guts: Self::Guts) -> Self;
}

/// Safely constructing values from their guts with possible failure.
pub trait TryFromGuts: Guts {
    type Error;

    /// Constructs a value from its guts, or fails.
    fn try_from_guts(guts: Self::Guts) -> Result<Self, Self::Error>;
}

#[cfg(feature = "never_type")]
impl<T> TryFromGuts for T
where
    T: FromGuts,
{
    type Error = !;

    fn try_from_guts(guts: Self::Guts) -> Result<Self, Self::Error> {
        Ok(Self::from_guts(guts))
    }
}

/// Unsafely constructing values from their guts without checking invariants.
pub trait FromGutsUnchecked: Guts {
    /// Constructs a value from its guts, without checking invariants.
    ///
    /// # Safety
    /// Depending on the invariants of `Self` this method may
    /// introduce unsafety by constructing from unchecked guts.
    unsafe fn from_guts_unchecked(guts: Self::Guts) -> Self;
}

impl<T> FromGutsUnchecked for T
where
    T: FromGuts,
{
    unsafe fn from_guts_unchecked(guts: Self::Guts) -> Self {
        Self::from_guts(guts)
    }
}
