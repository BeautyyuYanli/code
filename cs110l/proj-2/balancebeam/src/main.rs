mod request;
mod response;
mod rate_monitor;

use clap::Parser;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::task;
// use tokio::sync::{Mutex, RwLock};
use tokio::sync::Mutex;
use tokio::time::sleep;
use rate_monitor::RateMonitor;

/// Contains information parsed from the command-line invocation of balancebeam. The Clap macros
/// provide a fancy way to automatically construct a command-line argument parser.
#[derive(Parser, Debug)]
#[clap(about = "Fun with load balancing")]
struct CmdOptions {
    #[clap(
        short,
        long,
        help = "IP/port to bind to",
        default_value = "0.0.0.0:1100"
    )]
    bind: String,
    #[clap(short, long, help = "Upstream host to forward requests to")]
    upstream: Vec<String>,
    #[clap(
        long,
        help = "Perform active health checks on this interval (in seconds, 0 = doesn't check)",
        default_value = "0"
    )]
    active_health_check_interval: usize,
    #[clap(
    long,
    help = "Path to send request to for active health checks",
    default_value = "/"
    )]
    active_health_check_path: String,
    #[clap(
        long,
        help = "Maximum number of requests to accept per IP per minute (0 = unlimited)",
        default_value = "0"
    )]
    max_requests_per_minute: usize,
}

/// Contains information about the state of balancebeam (e.g. what servers we are currently proxying
/// to, what servers have failed, rate limiting counts, etc.)
///
/// You should add fields to this struct in later milestones.
struct ProxyState {
    /// How frequently we check whether upstream servers are alive (Milestone 4)
    active_health_check_interval: usize,
    /// Where we should send requests when doing active health checks (Milestone 4)
    active_health_check_path: String,
    /// Maximum number of requests an individual IP can make in a minute (Milestone 5)
    // max_requests_per_minute: usize,
    /// Rate monitor
    rate_monitor: Arc<RateMonitor>,
    /// Addresses of servers that we are proxying to
    upstream_addresses: Arc<Mutex<Vec<String>>>,
    /// Number of available upstream servers
    upstream_len: Arc<Mutex<usize>>
}

#[tokio::main]
async fn main() {
    // Initialize the logging library. You can print log messages using the `log` macros:
    // https://docs.rs/log/0.4.8/log/ You are welcome to continue using print! statements; this
    // just looks a little prettier.
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "debug");
    }
    pretty_env_logger::init();

    // Parse the command line arguments passed to this program
    let options = CmdOptions::parse();
    if options.upstream.len() < 1 {
        log::error!("At least one upstream server must be specified using the --upstream option.");
        std::process::exit(1);
    }

    // Start listening for connections
    let listener = match TcpListener::bind(&options.bind).await {
        Ok(listener) => listener,
        Err(err) => {
            log::error!("Could not bind to {}: {}", options.bind, err);
            std::process::exit(1);
        }
    };
    log::info!("Listening for requests on {}", options.bind);

    // Handle incoming connections
    let state = Arc::new(ProxyState {
        upstream_len: Arc::new(Mutex::new(options.upstream.len())),
        upstream_addresses: Arc::new(Mutex::new(options.upstream)),
        active_health_check_interval: options.active_health_check_interval,
        active_health_check_path: options.active_health_check_path,
        // max_requests_per_minute: options.max_requests_per_minute,
        rate_monitor: RateMonitor::new(options.max_requests_per_minute).await
    });
    if state.active_health_check_interval != 0 {
        task::spawn( active_health_check(state.clone()) );
    }
    loop{
        if let Ok(stream) = listener.accept().await {
            // Handle the connection!
            task::spawn( handle_connection(stream.0, state.clone()));
        }
    }
}

async fn connect_to_upstream(state: Arc<ProxyState>) -> Result<TcpStream, std::io::Error> {
    let mut rng = rand::rngs::StdRng::from_entropy();
    let mut upstream_len_lock = state.upstream_len.lock().await;
    let mut upstream_addr_lock = state.upstream_addresses.lock().await;
    let tcpstream = loop {
        if *upstream_len_lock == 0 {
            break Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable));
        }
        let upstream_idx = rng.gen_range(0, *upstream_len_lock);
        let upstream_ip = &upstream_addr_lock[upstream_idx];
        if let Ok(tcpstream) = TcpStream::connect(upstream_ip).await {
            break Ok(tcpstream);
        }
        else {
            *upstream_len_lock -= 1;
            upstream_addr_lock.swap(upstream_idx, *upstream_len_lock);
        }
    };
    drop(upstream_len_lock);
    drop(upstream_addr_lock);
    tcpstream
}

async fn send_response(client_conn: &mut TcpStream, response: &http::Response<Vec<u8>>) {
    let client_ip = client_conn.peer_addr().unwrap().ip().to_string();
    log::info!("{} <- {}", client_ip, response::format_response_line(&response));
    if let Err(error) = response::write_to_stream(&response, client_conn).await {
        log::warn!("Failed to send response to client: {}", error);
        return;
    }
}

async fn handle_connection(mut client_conn: TcpStream, state: Arc<ProxyState>) {
    let client_ip = client_conn.peer_addr().unwrap().ip().to_string();
    log::info!("Connection received from {}", client_ip);

    // Open a connection to a random destination server
    let mut upstream_conn = match connect_to_upstream(state.clone()).await {
        Ok(stream) => stream,
        Err(_error) => {
            let response = response::make_http_error(http::StatusCode::BAD_GATEWAY);
            send_response(&mut client_conn, &response).await;
            return;
        }
    };
    let upstream_ip = client_conn.peer_addr().unwrap().ip().to_string();

    // The client may now send us one or more requests. Keep trying to read requests until the
    // client hangs up or we get an error.
    loop {
        // Read a request from the client
        let mut request = match request::read_from_stream(&mut client_conn).await {
            Ok(request) => request,
            // Handle case where client closed connection and is no longer sending requests
            Err(request::Error::IncompleteRequest(0)) => {
                log::debug!("Client finished sending requests. Shutting down connection");
                return;
            }
            // Handle I/O error in reading from the client
            Err(request::Error::ConnectionError(io_err)) => {
                log::info!("Error reading request from client stream: {}", io_err);
                return;
            }
            Err(error) => {
                log::debug!("Error parsing request: {:?}", error);
                let response = response::make_http_error(match error {
                    request::Error::IncompleteRequest(_)
                    | request::Error::MalformedRequest(_)
                    | request::Error::InvalidContentLength
                    | request::Error::ContentLengthMismatch => http::StatusCode::BAD_REQUEST,
                    request::Error::RequestBodyTooLarge => http::StatusCode::PAYLOAD_TOO_LARGE,
                    request::Error::ConnectionError(_) => http::StatusCode::SERVICE_UNAVAILABLE,
                });
                send_response(&mut client_conn, &response).await;
                continue;
            }
        };
        log::info!(
            "{} -> {}: {}",
            client_ip,
            upstream_ip,
            request::format_request_line(&request)
        );

        // Check the rate limitation
        if let Err(_) = state.rate_monitor.try_request(&client_ip).await{
            send_response(&mut client_conn, &response::make_http_error(http::StatusCode::TOO_MANY_REQUESTS)).await;
            continue;
        }

        // Add X-Forwarded-For header so that the upstream server knows the client's IP address.
        // (We're the ones connecting directly to the upstream server, so without this header, the
        // upstream server will only know our IP, not the client's.)
        request::extend_header_value(&mut request, "x-forwarded-for", &client_ip);

        // Forward the request to the server
        if let Err(error) = request::write_to_stream(&request, &mut upstream_conn).await {
            log::error!("Failed to send request to upstream {}: {}", upstream_ip, error);
            let response = response::make_http_error(http::StatusCode::BAD_GATEWAY);
            send_response(&mut client_conn, &response).await;
            return;
        }
        log::debug!("Forwarded request to server");

        // Read the server's response
        let response = match response::read_from_stream(&mut upstream_conn, request.method()).await {
            Ok(response) => response,
            Err(error) => {
                log::error!("Error reading response from server: {:?}", error);
                let response = response::make_http_error(http::StatusCode::BAD_GATEWAY);
                send_response(&mut client_conn, &response).await;
                return;
            }
        };
        // Forward the response to the client
        send_response(&mut client_conn, &response).await;
        log::debug!("Forwarded response to client");
    }
}

async fn active_health_check(state: Arc<ProxyState>) {
    async fn check(addr: &str, path: &str) -> Result<(), std::io::Error>{
        let request = http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header("Host", addr)
                    .body(Vec::new())
                    .unwrap();
        let mut stream = TcpStream::connect(addr).await?;
        request::write_to_stream(&request, &mut stream).await?;
        match response::read_from_stream(&mut stream, &http::Method::GET).await {
            Err(_) => Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe)),
            Ok(res) => match res.status().as_u16() {
                200 => Ok(()),
                _ => Err(std::io::Error::new(std::io::ErrorKind::ConnectionRefused, res.status().as_str()))
            }
        }
    }
    loop {
        sleep(Duration::from_secs(state.active_health_check_interval as u64)).await;
        let mut upstream_len_lock = state.upstream_len.lock().await;
        let mut upstream_addr_lock = state.upstream_addresses.lock().await;
        let origin_upstream_len = *upstream_len_lock;
        let len = upstream_addr_lock.len();
        // check the previously available servers.
        for i in (0..origin_upstream_len).rev() {
            if let Err(_) = check(&upstream_addr_lock[i], &state.active_health_check_path).await {
                *upstream_len_lock -= 1;
                upstream_addr_lock.swap(*upstream_len_lock, i);
            }
        }
        // check the previously unavailable servers.
        for i in origin_upstream_len..len {
            if let Ok(_) = check(&upstream_addr_lock[i], &state.active_health_check_path).await {
                upstream_addr_lock.swap(*upstream_len_lock, i);
                *upstream_len_lock += 1;
            }
        }
        drop(upstream_len_lock);
        drop(upstream_addr_lock);
    }
}