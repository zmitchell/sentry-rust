use std::{collections::BTreeMap, time::SystemTime};

use serde::{Deserialize, Serialize};

use super::v7::{Context, Event, Level};

/// Represents feedback from a user.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Feedback {
    /// The user's contact email
    pub contact_email: Option<String>,
    /// The user's name
    pub name: Option<String>,
    /// The feedback from the user
    pub message: String,
}

impl Feedback {
    pub(crate) fn to_context(&self) -> Context {
        // let map = {
        //     let mut map = BTreeMap::new();
        //     if let Some(ref email) = self.contact_email {
        //         map.insert("contact_email".to_string(), email.clone());
        //     }
        //     if let Some(ref name) = self.name {
        //         map.insert("name".to_string(), name.clone());
        //     }
        //     map.insert("message".to_string(), self.message.clone());
        //     map
        // };
        Context::Feedback(Box::new(self.clone()))
    }

    pub(crate) fn to_new_event(&self) -> Event<'static> {
        let id = uuid::Uuid::new_v4();
        let map = {
            let mut map = BTreeMap::new();
            map.insert("feedback".to_string(), self.to_context());
            map
        };
        Event {
            event_id: id,
            level: Level::Info,
            timestamp: SystemTime::now(),
            contexts: map,
            ..Default::default()
        }
    }
}
