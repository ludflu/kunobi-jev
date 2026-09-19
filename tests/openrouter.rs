//! OpenRouter backend: Decisions path, request shape and parsed answers.
//!
//! Live smoke test (ignored by default):
//!
//! ```sh
//! # Pass the key for this command only (no need to export it in your profile):
//! OPENROUTER_API_KEY="$OPENROUTER_API_KEY" cargo test --test openrouter smoke -- --ignored --nocapture
//! ```

use kunobi_jev::{Client, Questions, SystemOneRequest, noul};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn openrouter_client(server: &MockServer) -> Client {
    Client::builder()
        .openrouter()
        .api_key("sk-or-test")
        .base_url(server.uri())
        .build()
        .unwrap()
}

#[tokio::test]
async fn decisions_endpoint_accepts_the_same_payload_shape() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/alpha/decisions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "model": "typesafe/jev-1.13-20260917",
            "answers": {
                "billing": {"type": "noul", "noul": 0.88}
            },
            "usage": {"input_tokens": 40, "output_tokens": 6, "cost": 0.00001}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let mut questions = Questions::new();
    let billing = questions.add("billing", noul("Is this about billing?"));
    let result = openrouter_client(&server)
        .system_one(SystemOneRequest::new("Double charge on my card.", questions))
        .await
        .unwrap();

    assert_eq!(result.answer(&billing).unwrap().noul, 0.88);
    let sent = &server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&sent.body).unwrap();
    assert_eq!(body["model"], "~typesafe/jev-latest");
    assert_eq!(body["state"], "Double charge on my card.");
}

#[tokio::test]
async fn models_list_is_unsupported() {
    let server = MockServer::start().await;
    let err = openrouter_client(&server)
        .models()
        .list()
        .await
        .unwrap_err();
    assert!(err.to_string().contains("Models API"), "{err}");
}

#[tokio::test]
#[ignore = "calls the live OpenRouter API; needs OPENROUTER_API_KEY"]
async fn smoke_decisions_round_trip() {
    let client = Client::openrouter().expect("set OPENROUTER_API_KEY to run the smoke test");

    let mut questions = Questions::new();
    let billing = questions.add("billing", noul("Is this about billing?"));
    let result = client
        .system_one(SystemOneRequest::new(
            "Double charge on my card.",
            questions,
        ))
        .with_response()
        .await
        .unwrap();

    let answer = result.data.answer(&billing).unwrap();
    assert!((0.0..=1.0).contains(&answer.noul));
    assert!(
        result.data.usage.input_tokens.is_some_and(|t| t > 0),
        "expected usage metadata from OpenRouter"
    );
    println!(
        "request {:?}: billing noul={:.3}, input_tokens={:?}",
        result.request_id,
        answer.noul,
        result.data.usage.input_tokens
    );
}
