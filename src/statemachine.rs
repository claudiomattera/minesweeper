// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::Mouse;

use crate::debug;

use crate::graphics::DrawColors;

mod mainmenu;
use mainmenu::MainMenuState;

mod ingame;
use ingame::InGameState;

pub static mut STATE_MACHINE: Machine = Machine {
    states_stack: [State::Initial; MAX_STATES_COUNT],
    current_state_index: 0,
};

/// Maximal number of states in the states stack
pub const MAX_STATES_COUNT: usize = 32;

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
    states_stack: [State; MAX_STATES_COUNT],
    current_state_index: usize,
}

impl Machine {
    pub fn draw(&self) {
        let draw_colors = DrawColors::new();
        let original_colors = draw_colors.get();
        for i in 0..(self.current_state_index + 1) {
            self.states_stack[i].draw();
            draw_colors.set(original_colors);
        }
    }

    pub fn update(&mut self, mouse: Option<&Mouse>) {
        match self.states_stack[self.current_state_index].update(mouse) {
            Transition::Switch(state) => {
                self.states_stack[self.current_state_index] = state;
            }
            Transition::Push(state) => {
                debug!(
                    "Pushing state to stack over {}th state",
                    self.current_state_index
                );
                self.current_state_index += 1;
                self.states_stack[self.current_state_index] = state;
                debug!("Current state: {}",self.states_stack[self.current_state_index].name());
            }
            Transition::Pop => {
                debug!("Popping {}th state from stack", self.current_state_index);
                self.current_state_index -= 1;
                debug!("Current state: {}",self.states_stack[self.current_state_index].name());
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
    MainMenu(MainMenuState),
    InGame(InGameState),
}

impl State {
    pub fn name(&self) -> &'static str {
        match self {
            State::Initial => "initial",
            State::MainMenu(_) => "mainmenu",
            State::InGame(_) => "ingame",
        }
    }

    pub fn draw(&self) {
        match self {
            State::Initial => {}
            State::MainMenu(s) => s.draw(),
            State::InGame(s) => s.draw(),
        }
    }

    pub fn update(self, mouse: Option<&Mouse>) -> Transition {
        match self {
            State::Initial => Transition::Push(State::MainMenu(MainMenuState::new())),
            State::MainMenu(state) => state.update(mouse),
            State::InGame(state) => state.update(mouse),
        }
    }
}
