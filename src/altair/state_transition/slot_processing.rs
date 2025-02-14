//! WARNING: This file was derived by the `gen-spec` utility. DO NOT EDIT MANUALLY.
use crate::altair as spec;
use crate::primitives::Slot;
use crate::state_transition::{Context, Error, Result};
use spec::{process_epoch, BeaconState};
use ssz_rs::prelude::*;
pub fn process_slots<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    slot: Slot,
    context: &Context,
) -> Result<()> {
    if state.slot >= slot {
        return Err(Error::TransitionToPreviousSlot {
            requested: slot,
            current: state.slot,
        });
    }
    while state.slot < slot {
        process_slot(state, context)?;
        if (state.slot + 1) % context.slots_per_epoch == 0 {
            process_epoch(state, context)?;
        }
        state.slot += 1;
    }
    Ok(())
}
pub fn process_slot<
    const SLOTS_PER_HISTORICAL_ROOT: usize,
    const HISTORICAL_ROOTS_LIMIT: usize,
    const ETH1_DATA_VOTES_BOUND: usize,
    const VALIDATOR_REGISTRY_LIMIT: usize,
    const EPOCHS_PER_HISTORICAL_VECTOR: usize,
    const EPOCHS_PER_SLASHINGS_VECTOR: usize,
    const MAX_VALIDATORS_PER_COMMITTEE: usize,
    const SYNC_COMMITTEE_SIZE: usize,
>(
    state: &mut BeaconState<
        SLOTS_PER_HISTORICAL_ROOT,
        HISTORICAL_ROOTS_LIMIT,
        ETH1_DATA_VOTES_BOUND,
        VALIDATOR_REGISTRY_LIMIT,
        EPOCHS_PER_HISTORICAL_VECTOR,
        EPOCHS_PER_SLASHINGS_VECTOR,
        MAX_VALIDATORS_PER_COMMITTEE,
        SYNC_COMMITTEE_SIZE,
    >,
    context: &Context,
) -> Result<()> {
    let previous_state_root = state.hash_tree_root()?;
    let root_index = state.slot % context.slots_per_historical_root as u64;
    state.state_roots[root_index as usize] = previous_state_root;
    if state.latest_block_header.state_root == Node::default() {
        state.latest_block_header.state_root = previous_state_root;
    }
    let previous_block_root = state.latest_block_header.hash_tree_root()?;
    let root_index = state.slot % context.slots_per_historical_root as u64;
    state.block_roots[root_index as usize] = previous_block_root;
    Ok(())
}
