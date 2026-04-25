//! BRP method handlers for the `ratchet_*` namespace.

use bevy::{
    prelude::*,
    remote::{BrpError, BrpResult, error_codes},
};
use bevy_motiongfx::{
    prelude::*,
    world::{MotionGfxWorld, TimelineId},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The method path for a `ratchet.get_timeline` request.
pub const RATCHET_GET_TIMELINE_METHOD: &str = "ratchet.get_timeline";

/// The method path for a `ratchet.seek_timeline` request.
pub const RATCHET_SEEK_TIMELINE_METHOD: &str = "ratchet.seek_timeline";

/// The method path for a `ratchet.start_player` request.
pub const RATCHET_START_PLAYER_METHOD: &str = "ratchet.start_player";

/// The method path for a `ratchet.stop_player` request.
pub const RATCHET_STOP_PLAYER_METHOD: &str = "ratchet.stop_player";

/// `ratchet.get_timeline`: Get the current timeline offset and status.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RatchetGetTimelineParams {
    pub entity: Entity,
}

/// `ratchet.seek_timeline`: Seeks towards a given time offset in timeline.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RatcheSeekTimelineParams {
    pub entity: Entity,
    pub offset: f32,
}

/// `ratchet.start_player`: Starts the player.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RatchetStartPlayerParams {
    pub entity: Entity,
}

/// `ratchet.start_player`: Stops the player.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RatchetStopPlayerParams {
    pub entity: Entity,
}

/// A helper function used to parse a `serde_json::Value`.
pub fn parse<T: for<'de> Deserialize<'de>>(value: Value) -> Result<T, BrpError> {
    serde_json::from_value(value).map_err(|err| BrpError {
        code: error_codes::INVALID_PARAMS,
        message: err.to_string(),
        data: None,
    })
}

/// A helper function used to parse a `serde_json::Value` wrapped in an `Option`.
pub fn parse_some<T: for<'de> Deserialize<'de>>(value: Option<Value>) -> Result<T, BrpError> {
    match value {
        Some(value) => parse(value),
        None => Err(BrpError {
            code: error_codes::INVALID_PARAMS,
            message: String::from("Params not provided"),
            data: None,
        }),
    }
}

/// A helper function for returning a BrpError if a timeline is missing.
fn no_timeline_err(entity: Entity) -> BrpError {
    BrpError {
        code: error_codes::INVALID_PARAMS,
        message: format!("No timeline found for TimelineId on entity {entity:?}"),
        data: None,
    }
}

/// A helper function for getting the TimelineId from an entity.
fn get_timeline_id(world: &World, entity: Entity) -> Result<TimelineId, BrpError> {
    world
        .get::<TimelineId>(entity)
        .copied()
        .ok_or_else(|| BrpError {
            code: error_codes::INVALID_PARAMS,
            message: format!("Entity {entity:?} has no TimelineId component"),
            data: None,
        })
}

/// The response to a `ratchet.get_timeline` request.
#[derive(Default, Serialize, Deserialize)]
struct RatchetGetTimelineResponse {
    /// The current time of the current track.
    curr_time: f32,
    /// The target time of the target track.
    target_time: f32,
    /// The index of the current track.
    curr_index: usize,
    /// The index of the target track.
    target_index: usize,
}

/// handle a `ratchet.get_timeline` request coming from a client.
pub fn process_ratchet_get_timeline_request(
    In(params): In<Option<Value>>,
    world: &World,
) -> BrpResult {
    let RatchetGetTimelineParams { entity } = parse_some(params)?;
    let timeline_id = get_timeline_id(world, entity)?;

    let motiongfx = world.resource::<MotionGfxWorld>();
    let timeline = motiongfx
        .get_timeline(&timeline_id)
        .ok_or_else(|| no_timeline_err(entity))?;

    serde_json::to_value(RatchetGetTimelineResponse {
        curr_time: timeline.curr_time(),
        target_time: timeline.target_time(),
        curr_index: timeline.curr_index(),
        target_index: timeline.target_index(),
    })
    .map_err(BrpError::internal)
}

/// Handle a `ratchet.seek_timeline` request coming from a client.
pub fn process_ratchet_seek_timeline_request(
    In(params): In<Option<Value>>,
    world: &mut World,
) -> BrpResult {
    let RatcheSeekTimelineParams { entity, offset } = parse_some(params)?;
    let timeline_id = { get_timeline_id(world, entity)? };

    let mut motiongfx = world.resource_mut::<MotionGfxWorld>();
    let timeline = motiongfx
        .get_timeline_mut(&timeline_id)
        .ok_or_else(|| no_timeline_err(entity))?;

    timeline.set_target_time(offset);
    Ok(Value::Null)
}

fn ratchet_set_player(entity: Entity, world: &mut World, playing: bool) -> BrpResult {
    if world.get::<RealtimePlayer>(entity).is_some() {
        world
            .get_mut::<RealtimePlayer>(entity)
            .unwrap()
            .set_playing(playing);
        return Ok(Value::Null);
    }

    Err(BrpError {
        code: error_codes::INVALID_PARAMS,
        message: format!(
            "Entity {entity:?} has neither RealtimePlayer nor a FixedRatePlayer component"
        ),
        data: None,
    })
}

/// Handle a `ratchet.start_player` request coming from a client.
pub fn process_ratchet_start_player_request(
    In(params): In<Option<Value>>,
    world: &mut World,
) -> BrpResult {
    let RatchetStartPlayerParams { entity } = parse_some(params)?;
    ratchet_set_player(entity, world, true)
}

/// Handle a `ratchet.stop_player` request coming from a client.
pub fn process_ratchet_stop_player_request(
    In(params): In<Option<Value>>,
    world: &mut World,
) -> BrpResult {
    let RatchetStopPlayerParams { entity } = parse_some(params)?;
    ratchet_set_player(entity, world, false)
}
