use askama::Template;
use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{uri::Authority, StatusCode, Uri},
    response::{Html, Redirect},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::{env, path::PathBuf};
use tower_http::{services::ServeDir, BoxError};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Ports {
    http: u16,
    https: u16,
}

#[allow(dead_code)]
async fn redirect_http_to_https(ports: Ports, addr: SocketAddr) {
    fn make_https(host: &str, uri: Uri, https_port: u16) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();
        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);
        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }
        let authority: Authority = host.parse()?;
        let bare_host = match authority.port() {
            Some(port_struct) => authority
                .as_str()
                .strip_suffix(port_struct.as_str())
                .unwrap()
                .strip_suffix(':')
                .unwrap(),
            None => authority.as_str(),
        };
        parts.authority = Some(format!("{bare_host}:{https_port}").parse()?);
        Ok(Uri::from_parts(parts)?)
    }
    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(&host, uri, ports.https) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };
    let addr = SocketAddr::from((addr.ip(), ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> Html<String> {
    let template = IndexTemplate {};
    Html(template.render().unwrap())
}

#[tokio::main]
async fn main() {
    dotenv().expect("expected dotenv");
    let ports;
    let listener;
    if env::var("PRODUCTION").unwrap_or("".to_string()) == "" {
        ports = Ports {
            http: 8080,
            https: 8443,
        };
        listener = SocketAddr::from(([0, 0, 0, 0], ports.https));
    } else {
        ports = Ports {
            http: 80,
            https: 443,
        };
        listener = SocketAddr::from(([0, 0, 0, 0], ports.https));
    }

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::spawn(redirect_http_to_https(ports, listener));

    let config = RustlsConfig::from_pem_file(
        PathBuf::from("./").join("private").join("cert.pem"),
        PathBuf::from("./").join("private").join("key.pem"),
    )
    .await
    .unwrap();

    tracing::debug!("listening on {}", listener);
    axum_server::bind_rustls(listener, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
