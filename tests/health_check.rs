#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
}
// Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Filed to bind address");
    let _ = tokio::spawn(server);
}
