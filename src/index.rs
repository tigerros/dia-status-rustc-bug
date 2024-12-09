use rinja::Template;
use crate::Items;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub items: Items
}