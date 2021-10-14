// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::unsync::Lazy;

use crate::mouse::Mouse;

use crate::debug;

mod initial;

mod ingame;
use ingame::InGameState;

pub static mut STATE_MACHINE: Lazy<Machine> = Lazy::new(|| {
    Machine {
        states_stack: vec![State::Initial],
    }
});

/// Main stackable state machine
///
/// The game state is modelled as a stackable state machine.
/// The current state is the one on top of the stack.
/// Three kinds of transition can happen:
///
/// * The current state can switch to a different state;
/// * A new state can be pushed to the stack;
/// * The current state can be popped from the stack.
pub struct Machine {
    states_stack: Vec<State>,
}

impl Machine {
    pub fn draw(&self) {
        for state in &self.states_stack {
            state.draw();
        }
    }

    pub fn update(&mut self, mouse: &Mouse) {
        let state: &mut State = self.states_stack.iter_mut().last().expect("Empty state machine!!!");
        let transition: Transition = state.update(mouse);
        match transition {
            Transition::Switch(state) => {
                let _ = self.states_stack.pop();
                self.states_stack.push(state);
            }
            Transition::Push(state) => {
                debug!("Pushing new state to stack");
                self.states_stack.push(state);
                debug!(
                    "Current state: {}",
                    self.states_stack.iter().last().unwrap().name()
                );
            }
            Transition::Pop => {
                debug!("Popping state from stack");
                let _ = self.states_stack.pop();
                debug!(
                    "Current state: {}",
                    self.states_stack.iter().last().unwrap().name()
                );
            }
        }
    }
}

pub enum Transition {
    Switch(State),
    Push(State),
    Pop,
}

#[derive(Clone, Copy)]
pub enum State {
    Initial,
    InGame(InGameState),
}

impl State {
    pub fn name(&self) -> &'static str {
        match self {
            State::Initial => "initial",
            State::InGame(_) => "in_game",
        }
    }

    pub fn draw(&self) {
        match self {
            State::Initial => {},
            State::InGame(s) => s.draw(),
        }
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        match self {
            State::Initial => {
                Transition::Push(State::InGame(InGameState::new()))
            }
            State::InGame(state) => state.update(mouse),
        }
    }
}
