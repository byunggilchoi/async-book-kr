// ANCHOR: imports
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
// ANCHOR_END: imports

// ANCHOR: timer_decl
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

/// future와 대기하는 스레드가 공유할 상태 데이터
struct SharedState {
    /// 수면 시간이 경과했는지 여부
    completed: bool,

    /// `TimerFuture`가 작동하는 task를 위한 waker
    /// 스레드는 `completed = true`를 설정한 후에 이를 사용하여 
    /// `TimerFuture`가 작동하는 task를 깨우고 `completed = true`를 확인해서
    /// 다음 단계로 갈 수 있습니다.
    waker: Option<Waker>,
}
// ANCHOR_END: timer_decl

// ANCHOR: future_for_timer
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 타이머가 완료되었는지를 확인하기 위해서 self.shared_state를 확인합니다.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            // SharedState의 완료여부를 확인해서 완료이면 Poll::Ready(()) 반환
            Poll::Ready(())
        } else {
            // 완료가 아니면 waker를 설정해서 스레드가 타이머가 완료되었을 때 지금의 task를 깨우고
            // future가 다시 poll해서 `completed = true`를 확인하도록 합니다.
            //
            // 매번 waker를 반복적으로 복제하는 것보다 한 번만 수행하는 것이 좋습니다.
            // 하지만 `TimeFuture`는 실행자의 task 간을 이동할 수 있어서
            // 이로 인해 잘못된 task를 가리켜서 `TimeFuture`가 올바르게 깨어나는 것을 막는
            // 부실한 waker가 생길 수 있습니다.
            //
            // 원래는 `Waker::will_wake` 함수를 사용해서 이를 확인할 수 있지만
            // 여기서는 예시를 간단하게 유지하기 위해 생략합니다.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
// ANCHOR_END: future_for_timer

// ANCHOR: timer_new
impl TimerFuture {
    /// 정해진 시간이 지나면 완료되는 새 `TimerFuture`를 만듭시다.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 새 스레드를 생성합시다.
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 타이머가 완료되었다는 것을 알리고 future가 폴링되는 마지막 task가 남아있으면 깨웁니다.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
// ANCHOR_END: timer_new

#[test]
fn block_on_timer() {
    futures::executor::block_on(async {
        TimerFuture::new(Duration::from_secs(1)).await
    })
}
