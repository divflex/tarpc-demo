#[tarpc::service]
pub trait Double {
    async fn double(x: i32) -> Result<i32, String>;
}
