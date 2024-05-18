use std::fmt;
use std::sync::Arc;

use crate::{Config, Result};

#[derive(Clone)]
pub struct Tasks(Arc<Config>);

impl Tasks {
    /// TODO.
    pub async fn create(&self) -> Result<()> {
        todo!()
    }

    /// TODO.
    pub async fn list(&self) -> Result<()> {
        todo!()
    }

    /// TODO.
    pub async fn run(&self) -> Result<()> {
        todo!()
    }

    /// TODO.
    pub async fn delete(&self) -> Result<()> {
        todo!()
    }
}

impl fmt::Debug for Tasks {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
