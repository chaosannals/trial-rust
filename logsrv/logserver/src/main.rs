mod server;

fn main() -> std::io::Result<()> {
    let server = server::LogServer::new("127.0.0.1", 22222);
    server.serve()
}
