mod index;

use std::collections::HashMap;
use rinja::Template;
use crate::index::IndexTemplate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Online,
    Outage
}

impl Status {
    pub const fn is_online(&self) -> bool {
        matches!(self, Self::Online)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Service(Status),
    Category(Items)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items(pub HashMap<String, Item>);

impl Items {
    /// Whether all services are online.
    pub fn is_online(&self) -> bool {
        for item in self.0.values() {
            match item {
                Item::Service(status) => if !status.is_online() {
                    return false;
                }
                Item::Category(items) => if !items.is_online() {
                    return false;
                }
            }
        }

        true
    }

    /// Whether all services are offline.
    pub fn is_offline(&self) -> bool {
        for item in self.0.values() {
            match item {
                Item::Service(status) => if status.is_online() {
                    return false;
                }
                Item::Category(items) => if !items.is_offline() {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let items = Items(HashMap::from([
        ("ORG".to_string(), Item::Service(Status::Online)),
        ("ROB".to_string(), Item::Service(Status::Online)),
    ]));

    let template = IndexTemplate { items };

    println!("{:?}", template.render());
}
