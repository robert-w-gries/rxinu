use core::{future::Future, pin::Pin};
use core::task::{Context, Poll};

pub async fn yield_now() {
    YieldNow(true).await;
}

struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.0 {
            false => Poll::Ready(()),
            true => {
                self.0 = false;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}
