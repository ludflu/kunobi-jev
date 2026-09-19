//! Which API hosts Jev for this client.

/// Where [`Client::system_one`](super::Client::system_one) sends requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Backend {
    /// The [TypeSafe](https://docs.typesafe.ai) System One API.
    #[default]
    TypeSafe,
    /// [OpenRouter](https://openrouter.ai) Decisions API for Jev.
    OpenRouter,
}

impl Backend {
    pub(crate) fn env_api_key(self) -> &'static str {
        match self {
            Backend::TypeSafe => super::env::ENV_API_KEY,
            Backend::OpenRouter => super::env::ENV_OPENROUTER_API_KEY,
        }
    }

    pub(crate) fn env_base_url(self) -> &'static str {
        match self {
            Backend::TypeSafe => super::env::ENV_BASE_URL,
            Backend::OpenRouter => super::env::ENV_OPENROUTER_BASE_URL,
        }
    }

    pub(crate) fn env_default_model(self) -> &'static str {
        match self {
            Backend::TypeSafe => super::env::ENV_DEFAULT_MODEL,
            Backend::OpenRouter => super::env::ENV_OPENROUTER_DEFAULT_MODEL,
        }
    }

    pub(crate) fn default_base_url(self) -> &'static str {
        match self {
            Backend::TypeSafe => super::DEFAULT_BASE_URL,
            Backend::OpenRouter => super::OPENROUTER_DEFAULT_BASE_URL,
        }
    }

    pub(crate) fn default_model(self) -> &'static str {
        match self {
            Backend::TypeSafe => super::DEFAULT_MODEL,
            Backend::OpenRouter => super::OPENROUTER_DEFAULT_MODEL,
        }
    }

    pub(crate) fn system_one_path(self) -> &'static str {
        match self {
            Backend::TypeSafe => "/v1/systemone",
            Backend::OpenRouter => "/alpha/decisions",
        }
    }

    pub(crate) fn models_list_supported(self) -> bool {
        matches!(self, Backend::TypeSafe)
    }
}
