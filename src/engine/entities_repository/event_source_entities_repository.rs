#![cfg(feature = "streams")]

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use redis::{
    streams::{self, StreamMaxlen},
    Commands,
};

use crate::engine::component::{Component, ComponentType};

use super::entities_repository::EntitiesRepository;

pub struct RedisStreamEventSourcing {
    client: redis::Client,
}

const ENTITIES_STREAM: &str = "entities";
const COMPONENTS_STREAM: &str = "components";
const MAXLEN: streams::StreamMaxlen = StreamMaxlen::Approx(1000);

impl RedisStreamEventSourcing {
    pub fn new() -> RedisStreamEventSourcing {
        RedisStreamEventSourcing {
            client: redis::Client::open("redis://127.0.0.1/").expect("client"),
        }
    }
}

impl<'a> EntitiesRepository<'a> for RedisStreamEventSourcing {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>) {
        let _: () = self
            .client
            .xadd_maxlen(ENTITIES_STREAM, MAXLEN, "*", &[("entity_id", entity_id)])
            .expect("msg");

        let formatted: Vec<(String, String)> = components
            .iter()
            .map(|c| {
                (
                    serde_json::to_string(&c.get_identifier()).unwrap(),
                    serde_json::to_string(c).unwrap(),
                )
            })
            .collect();
        let _: () = self
            .client
            .xadd_maxlen(COMPONENTS_STREAM, MAXLEN, "*", &formatted[..])
            .expect("msg");
    }

    fn retrieve_entity_by_id(&self, entity_id: &String) -> Option<&Vec<Box<dyn Component>>> {
        let components: () = self
        .client
        .xrevrange_count(COMPONENTS_STREAM, "+", "-", 1)
    }

    fn retrieve_entity_by_id_mut(
        &mut self,
        entity_id: &String,
    ) -> Option<&mut Vec<Box<dyn Component>>> {
        todo!()
    }

    fn retrieve_entities_by_components(
        &self,
        components: &Vec<crate::engine::component::ComponentType>,
    ) -> Vec<String> {
        todo!()
    }

    fn retrieve_entity_component(
        &self,
        entity_id: &String,
        component_type: &crate::engine::component::ComponentType,
    ) -> Option<&dyn Component> {
        todo!()
    }

    fn retrieve_entity_component_mut(
        &mut self,
        entity_id: &String,
        component_type: &crate::engine::component::ComponentType,
    ) -> Option<&mut dyn Component> {
        todo!()
    }
}
