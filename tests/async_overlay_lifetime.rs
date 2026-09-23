use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

use visionkit::async_api::{block_on, AsyncOverlaySubjects};
use visionkit::LiveTextInteraction;

struct Deadline<F> {
    inner: F,
    until: Instant,
    timer_started: bool,
}

impl<F: Future + Unpin> Future for Deadline<F> {
    type Output = Option<F::Output>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(value) = Pin::new(&mut self.inner).poll(cx) {
            return Poll::Ready(Some(value));
        }
        if Instant::now() >= self.until {
            return Poll::Ready(None);
        }
        if !self.timer_started {
            self.timer_started = true;
            let waker = cx.waker().clone();
            let until = self.until;
            thread::spawn(move || {
                thread::sleep(until.saturating_duration_since(Instant::now()));
                waker.wake();
            });
        }
        Poll::Pending
    }
}

fn with_deadline<F: Future + Unpin>(inner: F, timeout: Duration) -> Deadline<F> {
    Deadline {
        inner,
        until: Instant::now() + timeout,
        timer_started: false,
    }
}

fn main() {
    let Ok(interaction) = LiveTextInteraction::new() else {
        println!("[skip] LiveTextInteraction is unavailable on this Mac");
        return;
    };

    let (subjects, subject_at) = {
        let overlay = AsyncOverlaySubjects::new(&interaction);
        (overlay.subjects(), overlay.subject_at(1.0, 1.0))
    };
    drop(interaction);

    let subject_at = block_on(with_deadline(subject_at, Duration::from_millis(300)));
    let subjects = block_on(with_deadline(subjects, Duration::from_millis(300)));

    assert!(subject_at.is_none_or(|result| result.is_ok()));
    assert!(subjects.is_none_or(|result| result.is_ok()));
    println!("pending overlay queries survived dropping the interaction");
}
