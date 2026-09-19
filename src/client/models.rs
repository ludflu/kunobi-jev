//! The Models API resource.

use reqwest::Method;
use serde::Deserialize;

use super::Client;
use super::call::{Call, DecodeFailure};
use crate::error::Error;
use crate::types::ModelCard;

/// Access to the Models API resource.
#[derive(Debug, Clone)]
pub struct Models {
    client: Client,
}

impl Models {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// List the models available to the account.
    ///
    /// Only supported on the TypeSafe backend. OpenRouter clients return
    /// [`Error::InvalidRequest`] when this call is awaited.
    pub fn list(&self) -> Call<Vec<ModelCard>> {
        let body = if self.client.inner.backend.models_list_supported() {
            Ok(None)
        } else {
            Err(Error::InvalidRequest(
                "The Models API is only available with the TypeSafe backend; OpenRouter Jev \
                 uses the Decisions API and does not expose GET /v1/models."
                    .into(),
            ))
        };
        Call::new(
            self.client.clone(),
            Method::GET,
            "/v1/models",
            body,
            unwrap_models,
        )
    }
}

fn unwrap_models(body: &[u8]) -> Result<Vec<ModelCard>, DecodeFailure> {
    #[derive(Deserialize)]
    struct Wire {
        models: Vec<ModelCard>,
    }

    serde_json::from_slice::<Wire>(body)
        .map(|wire| wire.models)
        .map_err(|err| DecodeFailure {
            message: "Unexpected response shape from GET /v1/models; expected { models: [...] }."
                .into(),
            source: Some(err),
        })
}
