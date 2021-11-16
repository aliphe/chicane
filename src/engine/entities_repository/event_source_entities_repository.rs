use redis::{
    streams::{self, StreamMaxlen},
    Commands, RedisError,
};
use serde::__private::de::StrDeserializer;

use crate::engine::component::Component;

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

fn build_redis_payload(entity_id: &String, payload: &String) -> Vec<(String, String)> {
    vec![
        ("entity_id".to_string(), entity_id.to_string()),
        ("payload".to_string(), payload.to_string()),
    ]
}

impl<'a> EntitiesRepository<'a> for RedisStreamEventSourcing {
    fn register_entity(&mut self, entity_id: String, components: Vec<Box<dyn Component>>) {
        let _: () = self
            .client
            .xadd_maxlen(ENTITIES_STREAM, MAXLEN, "*", &[("entity_id", &entity_id)])
            .expect("msg");

        let formatted = serde_json::to_string(&components).unwrap();
        let payload = build_redis_payload(&entity_id, &formatted);

        let _: () = self
            .client
            .xadd_maxlen(COMPONENTS_STREAM, MAXLEN, "*", &payload)
            .expect("msg");
    }

    fn retrieve_entity_by_id(&mut self, entity_id: &String) -> Option<Vec<Box<dyn Component>>> {
        let result: Result<String, RedisError> =
            self.client.xrevrange_count(COMPONENTS_STREAM, "+", "-", 1);
        match result {
            Ok(raw) => {
                let deserialized: Vec<Box<dyn Component>> = serde_json::from_str(&raw).unwrap();
                Option::Some(deserialized)
            }
            Err(_) => Option::None,
        }
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
        &mut self,
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
