use std::net::TcpListener;

fn spawn_app() -> String {
    let listerner = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listerner.local_addr().unwrap().port();
    let server = zero2prod::run(listerner).expect("Failed to spawn server app");

    let _ = actix_web::rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
