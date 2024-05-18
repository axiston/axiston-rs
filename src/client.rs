use std::fmt;
use std::sync::Arc;

use reqwest::Client as RwClient;

use crate::{Builder, Config, Result};

/// Minimal [`Axiston`] client.
///
/// #### Usage
///
/// ```rust,no_run
/// use axiston::Client;
///
/// # let _ = async {
/// let client = Client::default();
/// client.health().await?;
/// # };
/// ```
///
/// [`Axiston`]: https://axiston.com
pub struct Client {
    pub(crate) config: Arc<Config>,
}

impl Client {
    /// Creates a new [`Axiston`] client.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `AXISTON_BASE_URL` is set but is not a valid `URL`.
    /// - Panics if the environment variable `AXISTON_USER_AGENT` is set but is not a valid `String`.
    ///
    /// [`Axiston`]: https://axiston.com
    pub fn new(api_key: &str) -> Self {
        Builder::new(api_key).build()
    }

    /// Creates a new [`Axiston`] client builder.
    ///
    /// [`Axiston`]: https://axiston.com
    pub fn builder(api_key: &str) -> Builder {
        Builder::new(api_key)
    }

    /// Returns `Ok(())` if the service is healthy.
    pub async fn health(&self) -> Result<()> {
        // TODO.
        Ok(())
    }

    /// Returns the reference to the provided `API key`.
    #[inline]
    #[must_use]
    pub fn api_key(&self) -> &str {
        self.config.api_key.as_ref()
    }

    /// Returns the reference to the used `User-Agent` header value.
    #[inline]
    #[must_use]
    pub fn user_agent(&self) -> &str {
        self.config.user_agent.as_str()
    }

    /// Returns the reference to the used `base URL`.
    #[inline]
    #[must_use]
    pub fn base_url(&self) -> &str {
        self.config.base_url.as_str()
    }

    /// Returns the underlying [`reqwest::Client`].
    ///
    /// [`reqwest::Client`]: ReqwestClient
    #[inline]
    #[must_use]
    pub fn client(&self) -> &RwClient {
        &self.config.client
    }
}

impl Default for Client {
    /// Creates a new [`Client`] from environment variables.
    ///
    /// ### Panics
    ///
    /// - Panics if the environment variable `AXISTON_API_KEY` is not set.
    /// - Panics if the environment variable `AXISTON_BASE_URL` is set but is not a valid `URL`.
    /// - Panics if the environment variable `AXISTON_USER_AGENT` is set but is not a valid `String`.
    fn default() -> Self {
        Builder::default().build()
    }
}

impl fmt::Debug for Client {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.config, f)
    }
}

#[cfg(test)]
mod test {
    use crate::{Client, Result};

    #[test]
    fn create() -> Result<()> {
        let _ = Client::new("");
        let _ = Client::builder("").build();
        let _ = Client::default();
        Ok(())
    }

    #[tokio::test]
    async fn health() -> Result<()> {
        let client = Client::default();
        client.health().await?;
        Ok(())
    }
}
