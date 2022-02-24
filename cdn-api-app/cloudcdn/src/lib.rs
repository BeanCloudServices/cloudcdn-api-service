#![allow(
    missing_docs,
    trivial_casts,
    unused_variables,
    unused_mut,
    unused_imports,
    unused_extern_crates,
    non_camel_case_types
)]
#![allow(clippy::all)]

use async_trait::async_trait;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::task::{Context, Poll};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/cloudcdn/latest";
pub const API_VERSION: &'static str = "0.0.1";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateDistributionResponse {
    /// successful operation
    SuccessfulOperation(models::DistributionUpsert),
    /// Invalid input
    InvalidInput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetDistributionResponse {
    /// successful operation
    SuccessfulOperation(models::DistributionUpsert),
    /// Invalid input
    InvalidInput,
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create new distribution
    async fn create_distribution(
        &self,
        distribution_upsert_request: models::DistributionUpsertRequest,
        context: &C,
    ) -> Result<CreateDistributionResponse, ApiError>;

    /// Create new distribution
    async fn get_distribution(
        &self,
        id: uuid::Uuid,
        context: &C,
    ) -> Result<GetDistributionResponse, ApiError>;
}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {
    fn poll_ready(
        &self,
        _cx: &mut Context,
    ) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create new distribution
    async fn create_distribution(
        &self,
        distribution_upsert_request: models::DistributionUpsertRequest,
    ) -> Result<CreateDistributionResponse, ApiError>;

    /// Create new distribution
    async fn get_distribution(&self, id: uuid::Uuid) -> Result<GetDistributionResponse, ApiError>;
}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync>
where
    Self: Sized,
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
        ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create new distribution
    async fn create_distribution(
        &self,
        distribution_upsert_request: models::DistributionUpsertRequest,
    ) -> Result<CreateDistributionResponse, ApiError> {
        let context = self.context().clone();
        self.api()
            .create_distribution(distribution_upsert_request, &context)
            .await
    }

    /// Create new distribution
    async fn get_distribution(&self, id: uuid::Uuid) -> Result<GetDistributionResponse, ApiError> {
        let context = self.context().clone();
        self.api().get_distribution(id, &context).await
    }
}

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;

#[cfg(feature = "lambda")]
pub mod lambda;
