use argon2::{self, Config};
use chrono::prelude::*;
use rand::Rng;
use std::{env, future};
use warp::{http::StatusCode, Filter};

use crate::{
    store::Store,
    types::account::{Account, AccountId, Session},
};

// ═══════════════════════════════ REGISTER ═════════════════════════════
pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_account = hash(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_account,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account created", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// ═════════════════════════════════ HASH ═══════════════════════════════
pub fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

// ═════════════════════════════════ LOGIN ═════════════════════════════════
pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            // ──────────────────────────────────────────────────────────────────────
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("Failed to get account id"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
            // ──────────────────────────────────────────────────────────────────────
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// ════════════════════════════ VERIFY_PASSWORD ════════════════════════════
fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

// ══════════════════════════════  ISSUE_TOKEN ══════════════════════════════
fn issue_token(account_id: AccountId) -> String {
    let key = env::var("PASETO_KEY").unwrap();
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    // let state = serde_json::to_string(&account_id).expect("Failed to serialize state");
    // local_paseto(&state, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
    //     .expect("Failed to create token")

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder")
}

// ═════════════════════════════ VERIFY_TOKEN ═══════════════════════════
pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_errors::Error::CannotDecryptToken)?;

    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotDecryptToken)
}

// ═════════════════════════════════ AUTH ═══════════════════════════════
pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };

        future::ready(Ok(token))
    })
}
