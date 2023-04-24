#[tarpc::service]
pub trait Add {
    async fn add(x: i32, y: i32) -> i32;
}
