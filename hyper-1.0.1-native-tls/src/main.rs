use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_ansi(false)
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let accept_tls = configure_tls();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let stream = match accept_tls.accept(stream).await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error accepting TLS connection: {:?}", e);
                continue;
            }
        };

        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = hyper::server::conn::http2::Builder::new(
                hyper_util::rt::tokio::TokioExecutor::new(),
            )
            .serve_connection(io, service_fn(hello))
            .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

use native_tls::{Identity, TlsAcceptor};
use std::fs;
use tokio_native_tls::TlsAcceptor as TokioTlsAcceptor;

/* certificate.pfx generated with:

openssl req -newkey rsa:512 -nodes -keyout key.pem -x509 -days 365 -out certificate.pem -subj "/CN=localhost" && \
  openssl pkcs12 -export -out certificate.pfx -inkey key.pem -in certificate.pem -passout pass:foo -descert -macalg sha1

*/

fn configure_tls() -> TokioTlsAcceptor {
    let pkcs12 = fs::read("./certificate.pfx").unwrap();
    let identity = Identity::from_pkcs12(&pkcs12, "foo").unwrap();
    let acceptor = TlsAcceptor::builder(identity).build().unwrap();
    TokioTlsAcceptor::from(acceptor)
}
