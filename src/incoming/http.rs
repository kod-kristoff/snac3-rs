use anyhow::Context;
use tokio::net;

pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

pub struct HttpServer {
    listener: net::TcpListener,
    router: axum::Router,
}

impl HttpServer {
    pub async fn new(config: HttpServerConfig<'_>) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let router = axum::Router::new().layer(trace_layer);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { listener, router })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}
