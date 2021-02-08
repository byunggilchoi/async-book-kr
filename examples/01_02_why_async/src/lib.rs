#![cfg(test)]

use {
    futures::{
        executor::block_on,
        join,
    },
    std::thread,
};

fn download(_url: &str) {
    // ...
}

#[test]
// ANCHOR: get_two_sites
fn get_two_sites() {
    // 두 개의 스레드를 생성합니다.
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // 양 스레드가 작업을 마치는 것을 기다립니다.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
// ANCHOR_END: get_two_sites

async fn download_async(_url: &str) {
    // ...
}

// ANCHOR: get_two_sites_async
async fn get_two_sites_async() {
    // 완료될 때까지 비동기적으로 웹페이지들을 다운로드하는
    // 서로 다른 두 개의 "futures"를 만듭니다.
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // 동시에 두 futures를 모두 작동시킵니다.
    join!(future_one, future_two);
}
// ANCHOR_END: get_two_sites_async

#[test]
fn get_two_sites_async_test() {
    block_on(get_two_sites_async());
}
