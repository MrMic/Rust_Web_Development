#[warn(clippy::all)]
use clap::Parser;
// use config::Config;
use std::env;

use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod profanity;
mod routes;
mod store;
mod types;

// ______________________________________________________________________
/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "debug")]
    log_level: String,
    /// Username for the postgres database
    #[clap(long, default_value = "postgres")]
    database_user: String,
    /// Password for the postgres database
    #[clap(long, default_value = "")]
    database_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "172.23.0.2")]
    database_host: String,
    /// PORT number for the postgres database
    #[clap(long, default_value = "5432")]
    database_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    database_name: String,
}

// ╾────────────────────────────╼ WRAP Server ╾────────────────────────────╼
#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();

    if let Err(_) = env::var("BAD_WORDS_API_KEY") {
        panic!("BAD_WORDS_API_KEY not set");
    }
    if let Err(_) = env::var("PASETO_KEY") {
        panic!("PASETO_KEY not set");
    }
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(3030))
        .map_err(|e| handle_errors::Error::ParseError(e))?;

    // ╾──────────────────────────────╼ CONFIG ╾────────────────────────────╼
    /* let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();

    let config = config.try_deserialize::<Args>().unwrap(); */

    // ╾─────────────────────────╼ CLAP CLI PARSING ╾───────────────────────╼
    let args = Args::parse();

    // ━━━━━━━━━━━━━━━━━━━━━━━ LOGGING With WRAP // w/CONFIG ━━━━━━━━━━━━━━━━━
    /* let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web_dev={},warp={}",
            config.log_level, config.log_level, config.log_level
        )
    });

    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    ))
    .await; */

    // ╾──────────────────╼ LOGGING w/WRAP && w/CLAP -= CLI ╾──────────────────╼
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rustwebdev={},warp={}",
            args.log_level, args.log_level, args.log_level
        )
    });

    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        args.database_user,
        args.database_password,
        args.database_host,
        args.database_port,
        args.database_name
    ))
    .await
    .map_err(|e| handle_errors::Error::DatabaseQueryError(e))?;

    // let store = store::Store::new("postgres://postgres:pepere@172.23.0.3:5432/rustwebdev").await;

    // ______________________________________________________________________
    // WARN: Execute the migration files (migrations Folder).
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .map_err(|e| handle_errors::Error::MigrationError(e))?;

    // ______________________________________________________________________
    let store_filter = warp::any().map(move || store.clone());

    // let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        //Record an event when each span closes. This can be used to
        // time our route's durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec![Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    //         ╭──────────────────────────────────────────────────────────╮
    //         │                       WRAP Server                        │
    //         ╰──────────────────────────────────────────────────────────╯

    // let routes = add_question.recover(return_error);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        // .with(log)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}
