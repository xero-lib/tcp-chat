
fn exec() {
    let mut counter = Counter(0);

    loop {
        if let FakePoll::Ready(value) = counter.poll() {
            info!("Counter is ready! Got {} from it.", value);
            break;
        } else {
            trace!("Counter is pending :c");
        }
    }
}

enum FakePoll<T> {
    Ready(T),
    Pending,
}

trait FakeFuture {
    type Output;

    fn poll(&mut self) -> FakePoll<Self::Output>;
}

#[derive(Debug)]
struct Counter(u64);

impl FakeFuture for Counter {
    type Output = u64;

    #[tracing::instrument]
    fn poll(&mut self) -> FakePoll<u64> {
        if self.0 > 5 {
            return FakePoll::Ready(self.0);
        }
        self.0 += 1;
        trace!("still pending :>");
        FakePoll::Pending
    }
}
