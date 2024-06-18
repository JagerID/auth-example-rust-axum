use std::error::Error;

pub mod scylla;

pub trait DB<D> {
    async fn connect(port: u16) -> Self;

    async fn migrate(&self) -> Result<(), Box<dyn Error>>;
}
