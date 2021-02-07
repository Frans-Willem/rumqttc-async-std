use std::task::{Context, Poll};
use std::pin::Pin;
use std::future::Future;
use futures::future::FusedFuture;
use pin_project::pin_project;

#[pin_project]
pub struct CondFuture<F> {
    #[pin]
    f: Option<F>,
}

impl<F> CondFuture<F> {
    pub fn new(f: F, condition: bool) -> CondFuture<F> {
        if condition {
            CondFuture { f: Some(f) }
        } else {
            CondFuture { f: None }
        }
    }
}

impl<F: Future> Future for CondFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(f) = self.project().f.as_pin_mut() {
            f.poll(cx)
        } else {
            Poll::Pending
        }
    }
}

impl<F: FusedFuture> FusedFuture for CondFuture<F> {
    fn is_terminated(&self) -> bool {
        if let Some(f) = &self.f {
            f.is_terminated()
        } else {
            true
        }
    }
}

pub fn cond_fut<F>(f: F, condition: bool) -> CondFuture<F> {
    CondFuture::new(f, condition)
}
