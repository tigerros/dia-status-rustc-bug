use rinja::Template;
use crate::{Status, Items, Item};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub items: Items
}