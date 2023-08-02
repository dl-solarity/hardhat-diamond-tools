use actions::{
    merge::{MERGE_DESCRIPTION, MERGE_TASK},
    names::{NAMES_DESCRIPTION, NAMES_TASK},
};
use hardhat_bindings::{bindings::config::ActionType, config::task};
use wasm_bindgen::prelude::*;

mod actions;
mod node_bindings;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Tasks {
    _names: ActionType,
    _merge: ActionType,
}

/// The main entrypoint of the plugin. This file will be [`required`]
/// in [`hardhat.config.js`] and here all tasks will be created.
#[wasm_bindgen]
pub fn run() -> Tasks {
    let names_action_cb =
        task(NAMES_TASK, NAMES_DESCRIPTION).set_action(actions::names::names_action);
    let merge_action_cb =
        task(MERGE_TASK, MERGE_DESCRIPTION).set_action(actions::merge::merge_artifacts_action);

    Tasks {
        _names: names_action_cb,
        _merge: merge_action_cb,
    }
}
