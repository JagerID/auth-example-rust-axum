use std::error::Error;

pub mod scylla;

pub trait DB<D> {
    async fn connect(host: String, port: u16) -> Self;

    async fn migrate(&self) -> Result<(), Box<dyn Error>>;
}
