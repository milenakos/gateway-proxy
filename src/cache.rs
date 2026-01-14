use serde::Serialize;
use serde_json::Value as OwnedValue;
use twilight_model::gateway::OpCode;

use crate::model::JsonObject;

#[derive(Serialize)]
pub struct Payload<T> {
    pub d: T,
    pub op: OpCode,
    pub t: &'static str,
    pub s: usize,
}

pub struct Guilds;

pub struct CacheStats;

impl CacheStats {
    pub fn emojis(&self) -> usize {
        0
    }

    pub fn guilds(&self) -> usize {
        0
    }

    pub fn members(&self) -> usize {
        0
    }

    pub fn presences(&self) -> usize {
        0
    }

    pub fn channels(&self) -> usize {
        0
    }

    pub fn roles(&self) -> usize {
        0
    }

    pub fn unavailable_guilds(&self) -> usize {
        0
    }

    pub fn users(&self) -> usize {
        0
    }

    pub fn voice_states(&self) -> usize {
        0
    }
}

impl Guilds {
    pub const fn new() -> Self {
        Self
    }

    pub fn update<T>(&self, _value: T) {
        // no-op: caching disabled
    }

    pub fn stats(&self) -> CacheStats {
        CacheStats
    }

    pub fn get_ready_payload(
        &self,
        mut ready: JsonObject,
        sequence: &mut usize,
    ) -> Payload<JsonObject> {
        *sequence += 1;

        ready.insert(String::from("guilds"), OwnedValue::Array(vec![]));

        Payload {
            d: ready,
            op: OpCode::Dispatch,
            t: "READY",
            s: *sequence,
        }
    }

    pub fn get_guild_payloads<'a>(&'a self, _sequence: &'a mut usize) -> impl Iterator<Item = String> + 'a {
        std::iter::empty()
    }
}
