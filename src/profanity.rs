use std::env;

use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse(String);

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn check_profanity(content: String) -> Result<String, handle_errors::Error> {
    // We are already checking if the ENV VARIABLE is set inside main.rs
    // so safe to unwrap here
    let api_key = env::var("BAD_WORDS_API_KEY").unwrap();
    let api_layer_url = env::var("API_LAYER_URL").expect("API_LAYER_URL must be set");

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let res = client
        // .post("https://api.apilayer.com/bad_words?censor_character={censor_character}")
        .post(format!("{}/bad_words?censor_character=*", api_layer_url))
        .header("apikey", api_key)
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::MiddlewareReqwestAPIError(e))?;

    if !res.status().is_success() {
        let status = res.status().as_u16();
        let message = res.json::<APIResponse>().await.unwrap();

        let err = handle_errors::APILayerError {
            status,
            message: message.0,
        };

        if status < 500 {
            return Err(handle_errors::Error::ClientError(err));
        } else {
            return Err(handle_errors::Error::ServerError(err));
        }
    }

    match res.json::<BadWordResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(handle_errors::Error::ReqwestAPIError(e)),
    }
}

// ___________________________ TEST: PROFANITY TESTS ___________________________
#[cfg(test)]
mod profanity_tests {
    use super::{check_profanity, env};

    use mock_server::{MockServer, OneshotHandler};

    #[tokio::test]
    async fn run() {
        let handler = run_mock();
        censor_profane_words().await;
        no_profane_words().await;
        let _ = handler.sender.send(1);
    }

    fn run_mock() -> OneshotHandler {
        env::set_var("API_LAYER_URL", "http://127.0.0.1:3030");
        env::set_var("BAD_WORDS_API_KEY", "YES");

        let socket = "127.0.0.1:3030"
            .to_string()
            .parse()
            .expect("Not a valid address");
        let mock = MockServer::new(socket);
        mock.oneshot()
    }

    async fn censor_profane_words() {
        let content = "this is a shitty sentence".to_string();
        let censored_content = check_profanity(content).await;
        assert_eq!(
            censored_content.unwrap(),
            "this is a ****** sentence".to_string()
        );
    }

    async fn no_profane_words() {
        let content = "this is a good sentence".to_string();
        let censored_content = check_profanity(content).await;
        assert_eq!(censored_content.unwrap(), "");
    }
}
