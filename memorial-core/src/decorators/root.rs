use std::collections::HashMap;
use std::time::SystemTime;

use time::OffsetDateTime;

use crate::decorators::Decorator;
use crate::model::attributes;
use crate::model::handle::Handle;
use crate::model::tree::Node;

pub struct RootDecorator {
    pub title: String,
}

impl Decorator for RootDecorator {
    fn decorate(&self, tree: &mut Node) -> anyhow::Result<()> {
        tree.merge_attributes(
            &Handle::ROOT,
            HashMap::from([
                (
                    attributes::APP_VERSION.to_string(),
                    env!("CARGO_PKG_VERSION").to_string(),
                ),
                (
                    attributes::TIMESTAMP.to_string(),
                    OffsetDateTime::from(SystemTime::now())
                        .format(
                            &time::format_description::parse(
                                "[year]-[month]-[day] [hour]:[minute]:[second]",
                            )
                            .unwrap(),
                        )
                        .unwrap(),
                ),
                (attributes::TITLE.to_string(), self.title.clone()),
            ]),
        );
        Ok(())
    }
}
