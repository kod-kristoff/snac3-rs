use crate::domain::blog::{config::BlogConfig, ports::BlogService};

pub struct Service {
    config: BlogConfig,
}

impl BlogService for Service {
    fn host(&self) -> &str {
        &self.config.host
    }
}
