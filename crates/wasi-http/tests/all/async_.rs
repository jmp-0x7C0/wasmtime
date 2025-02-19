use super::*;
use test_programs_artifacts::*;
use wasmtime_wasi::preview2::command::Command;

foreach_http!(assert_test_exists);

async fn run(path: &str, server: &Server) -> Result<()> {
    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, path)?;
    let mut store = store(&engine, server);
    let mut linker = Linker::new(&engine);
    wasmtime_wasi_http::proxy::add_to_linker(&mut linker)?;
    let (command, _instance) = Command::instantiate_async(&mut store, &component, &linker).await?;
    let result = command.wasi_cli_run().call_run(&mut store).await?;
    result.map_err(|()| anyhow::anyhow!("run returned an error"))
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_get() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_GET_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_post() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_POST_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_large_post() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_LARGE_POST_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_put() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_PUT_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_invalid_version() -> Result<()> {
    let server = Server::http2()?;
    run(HTTP_OUTBOUND_REQUEST_INVALID_VERSION_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_invalid_header() -> Result<()> {
    let server = Server::http2()?;
    run(HTTP_OUTBOUND_REQUEST_INVALID_HEADER_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_unknown_method() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_UNKNOWN_METHOD_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_unsupported_scheme() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_UNSUPPORTED_SCHEME_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_invalid_port() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_INVALID_PORT_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_invalid_dnsname() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_INVALID_DNSNAME_COMPONENT, &server).await
}

#[test_log::test(tokio::test(flavor = "multi_thread"))]
async fn http_outbound_request_response_build() -> Result<()> {
    let server = Server::http1()?;
    run(HTTP_OUTBOUND_REQUEST_RESPONSE_BUILD_COMPONENT, &server).await
}
