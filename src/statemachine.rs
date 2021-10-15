// Copyright Claudio Mattera 2021.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::unsync::Lazy;

use crate::mouse::Mouse;

use crate::debug;

mod initial;
use initial::InitialState;

mod gameover;
use gameover::GameOverState;

mod gamewon;
use gamewon::GameWonState;

mod pause;
use pause::PauseState;

mod pregame;
use pregame::PreGameState;

mod ingame;
use ingame::InGameState;

pub static mut STATE_MACHINE: Lazy<Machine> = Lazy::new(|| {
    Machine {
        states_stack: vec![State::Initial(InitialState::new())],
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
        let state: State = self.states_stack
            .pop()
            .expect("Empty state machine!!!");
        let transition: Transition = state.update(mouse);

        match transition {
            Transition::Replace(state) => {
                self.states_stack.push(state);
            }
            Transition::Push(old_state, state) => {
                // First restore old state onto stack
                self.states_stack.push(old_state);

                debug!("Pushing new state to stack");
                self.states_stack.push(state);
                debug!(
                    "Current state: {}",
                    self.states_stack.iter().last().unwrap().name()
                );
            }
            Transition::Pop => {
                debug!("Popping state from stack");
                // Already done at the beginning of this function
                debug!(
                    "Current state: {}",
                    self.states_stack.iter().last().unwrap().name()
                );
            }
        }
    }
}

pub enum Transition {
    Replace(State),
    Push(State, State),
    Pop,
}

#[derive(Clone)]
pub enum State {
    Initial(InitialState),
    PreGame(PreGameState),
    InGame(InGameState),
    GameOver(GameOverState),
    GameWon(GameWonState),
    Pause(PauseState),
}

impl State {
    pub fn name(&self) -> &'static str {
        match self {
            State::Initial(_) => "initial",
            State::PreGame(_) => "pre_game",
            State::InGame(_) => "in_game",
            State::GameOver(_) => "game_over",
            State::GameWon(_) => "game_won",
            State::Pause(_) => "pause",
        }
    }

    pub fn draw(&self) {
        match self {
            State::Initial(s) => s.draw(),
            State::PreGame(s) => s.draw(),
            State::InGame(s) => s.draw(),
            State::GameOver(s) => s.draw(),
            State::GameWon(s) => s.draw(),
            State::Pause(s) => s.draw(),
        }
    }

    pub fn update(self, mouse: &Mouse) -> Transition {
        match self {
            State::Initial(state) => state.update(mouse),
            State::PreGame(state) => state.update(mouse),
            State::InGame(state) => state.update(mouse),
            State::GameOver(state) => state.update(mouse),
            State::GameWon(state) => state.update(mouse),
            State::Pause(state) => state.update(mouse),
        }
    }
}
