use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use std::time::Instant;

pub async fn logging_middleware(req: ServiceRequest, next: &dyn actix_service::Service) -> Result<ServiceResponse, Error> {
    let start = Instant::now();
    let res = next.call(req).await?;
    let duration = start.elapsed();
    println!("Request took: {:?}", duration);
    Ok(res)
}