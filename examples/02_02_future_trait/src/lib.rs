// ANCHOR: simple_future
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
// ANCHOR_END: simple_future

struct Socket;
impl Socket {
    fn has_data_to_read(&self) -> bool {
        // check if the socket is currently readable
        true
    }
    fn read_buf(&self) -> Vec<u8> {
        // Read data in from the socket
        vec![]
    }
    fn set_readable_callback(&self, _wake: fn()) {
        // register `_wake` with something that will call it
        // once the socket becomes readable, such as an
        // `epoll`-based event loop.
    }
}

// ANCHOR: socket_read
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // 소켓에 데이터가 있을 경우, 읽어서 버퍼에 넣은 후 반환
            Poll::Ready(self.socket.read_buf())
        } else {
            // 소켓에 데이터가 없는 경우,
            //
            // 데이터가 사용가능할 때 `wake`를 호출하도록 설정
            // 데이터가 사용가능할 때 `wake`가 호출되고 
            // 그 결과 이 `Future`의 사용자가 인지해서 `poll`을 실행해서
            // 데이터를 받음
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
// ANCHOR_END: socket_read

// ANCHOR: join
/// 두 개의 future가 모두 완료될 때까지 동시적으로 실행하는 SimpleFuture
///
/// 동시성은 각 future의 `poll` 호출이 독립적으로 이뤄질 수 있다는 것을 통해 달성됩니다.
///  이를 통해 각 future는 각자의 속도로 다음 단계로 갈 수 있습니다.
pub struct Join<FutureA, FutureB> {
    // 각 필드는 완료될 때까지 작동하는 future를 담고 있을 수 있습니다.
    // future가 이미 완료되었으면 그 필드는 `None`이 됩니다.
    // 이를 통해 완료된 뒤에도 계속 폴링을 하는 것을 막습니다.
    // `Future` 트레잇의 내용과 어긋나니까요.
    a: Option<FutureA>,
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // future `a`를 완료하려고 합니다.
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }

        // future `b`를 완료하려고 합니다.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() && self.b.is_none() {
            // 두 future가 모두 완료되어서 성공적으로 반환할 수 있습니다.
            Poll::Ready(())
        } else {
            // 하나 혹은 두 future 모두가 `Poll::Pending`을 반환하면 아직 덜 끝났다는 뜻입니다.
            // 그러면 변화가 있을 때 `wake()`를 호출할 것입니다. 
            Poll::Pending
        }
    }
}
// ANCHOR_END: join

// ANCHOR: and_then
/// 두 개의 future가 모두 완료될 때까지 하나씩 실행하는 SimpleFuture
//
// Note: 간단한 예시를 위해 `AndThenFut`은 두 future가 모두 생성될 때 사용가능하다고 가정합니다.
// 실제 `AndThen` 결합자는 `get_breakfast.and_then(|food| eat(food))`와 같이
// 첫번째 future의 결과값을 바탕으로 두번째 future를 생성할 수 있습니다.
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // 첫번째 future를 완료한 경우입니다.
                // 삭제하고 두 번째를 시작합시다.
                Poll::Ready(()) => self.first.take(),
                // 아직 첫번째 future를 완료하지 못한 경우입니다.
                Poll::Pending => return Poll::Pending,
            };
        }
        // 이제 첫번째 future는 끝났으니 두번째를 완료할 차례입니다.
        self.second.poll(wake)
    }
}
// ANCHOR_END: and_then

mod real_future {
use std::{
    future::Future as RealFuture,
    pin::Pin,
    task::{Context, Poll},
};

// ANCHOR: real_future
trait Future {
    type Output;
    fn poll(
        // `&mut self`가 아니라 `Pin<&mut Self>`입니다.
        self: Pin<&mut Self>,
        // `wake: fn()`가 아니라 `cx: &mut Context<'_>`입니다.:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
// ANCHOR_END: real_future

// ensure that `Future` matches `RealFuture`:
impl<O> Future for dyn RealFuture<Output = O> {
    type Output = O;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        RealFuture::poll(self, cx)
    }
}
}
