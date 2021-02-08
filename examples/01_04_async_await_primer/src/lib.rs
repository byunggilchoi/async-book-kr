#![cfg(test)]

use futures::executor::block_on;

mod first {
// ANCHOR: hello_world
// `block_on`은 필요한 future가 완료될 때까지 현재 스레드를 중단시킵니다.
// 다른 실행자(executor)들은 여러 future들을 같은 스레드에서 실행하는 
// 일정을 짜는 등 더 복잡한 방식을 제공합니다.
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 아무것도 출력하지 않습니다.
    block_on(future); // `future`가 실행되어서 "hello, world!"를 출력합니다.
}
// ANCHOR_END: hello_world

#[test]
fn run_main() { main() }
}

struct Song;
async fn learn_song() -> Song { Song }
async fn sing_song(_: Song) {}
async fn dance() {}

mod second {
use super::*;
// ANCHOR: block_on_each
fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
// ANCHOR_END: block_on_each

#[test]
fn run_main() { main() }
}

mod third {
use super::*;
// ANCHOR: block_on_main
async fn learn_and_sing() {
    // 노래를 배울 때까지 기다립니다.
    // `block_on` 대신 `.awiat`을 사용해서 스레드를 멈추게 하는 것을 막습니다.
    // 그래서 `dance`를 동시에 실행할 수 있습니다.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`은 `.await`와 비슷하지만 여러 개의 future를 동시에 기다릴 수 있습니다.
    // `learn_and_sing` future에 막혀 있다면 `dance` future가 현재의 스레드를 접수할 것입니다.
    // `dance`가 막혀 있다면 `learn_and_sing`이 다시 스레드를 접수하겠죠.
    // 두 future가 모두 막혀있으면 `async_main`이 막히고 실행자에게 권한이 갈 것입니다.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
// ANCHOR_END: block_on_main

#[test]
fn run_main() { main() }
}
