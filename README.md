# guts

<!-- [![Build Status](http://img.shields.io/travis/regexident/guts.svg?style=flat-square)](https://travis-ci.org/regexident/guts) -->
[![Downloads](https://img.shields.io/crates/d/guts.svg?style=flat-square)](https://crates.io/crates/guts/)
[![Version](https://img.shields.io/crates/v/guts.svg?style=flat-square)](https://crates.io/crates/guts/)
[![License](https://img.shields.io/crates/l/guts.svg?style=flat-square)](https://crates.io/crates/guts/)

## Synopsis

Traits for constructing/destructuring from/into a type's internal guts.

## Example

```rust
mod state_machine {
    use guts::{HasGuts, FromGutsUnchecked};

    /// A State machine's internal state.
    pub enum State {
        Off,
        On,
    }

    /// A State machine that hides its internal state.
    pub struct StateMachine {
        state: State,
    }

    impl Default for StateMachine {
        /// Creates a state machine in the only allowed initial state: `Off`.
        fn default() -> Self {
            Self { state: State::Off }
        }
    }

    impl HasGuts for StateMachine {
        type Guts = State;
    }

    impl FromGutsUnchecked for StateMachine {
        /// Creates a state machine in an arbitrary state, unsafely.
        unsafe fn from_guts_unchecked(guts: Self::Guts) -> Self {
            Self { state: guts }
        }
    }
}

use guts::FromGutsUnchecked;
use state_machine::{State, StateMachine};

// A machine can easily be safely created in its initial state:
let machine = StateMachine::default();

// To create a machine in a non-initial state `unsafe { … }` is required:
let machine = unsafe { StateMachine::from_guts_unchecked(State::On) };
```

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0).
