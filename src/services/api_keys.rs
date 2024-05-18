use std::fmt;
use std::sync::Arc;

use crate::{Config, Result};

#[derive(Clone)]
pub struct ApiKeys(Arc<Config>);

impl ApiKeys {
    /// TODO.
    pub async fn create(&self) -> Result<()> {
        todo!()
    }

    /// TODO.
    pub async fn list(&self) -> Result<()> {
        todo!()
    }

    /// TODO.
    pub async fn delete(&self) -> Result<()> {
        todo!()
    }
}

impl fmt::Debug for ApiKeys {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
