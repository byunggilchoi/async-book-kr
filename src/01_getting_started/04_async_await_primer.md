# `async`/`.await` 입문

`async`/`.await`는 동기 코드처럼 보이는 비동기 함수를 작성하기 위한 Rust의 빌트인 도구입니다. `async`는 코드 블록을 `Future`라는 트레잇을 구현하는 상태 머신으로 변환합니다. 동기 메서드에서 차단된 함수를 호출하면 전체 스레드가 차단되지만 차단된 `Future`는 스레드를 제어하여 다른 `Future`를 실행시킵니다.

`Cargo.toml` 파일에 종속성을 추가해봅시다.

```toml
{{#include ../../examples/01_04_async_await_primer/Cargo.toml:9:10}}
```

비동기 함수를 만들기 위해 `async fn` 함수를 사용할 수 있습니다.

```rust,edition2018
async fn do_something() { /* ... */ }
```

`async fn`에서 반환하는 값은 `Future` 타입입니다. 어떤 일이 일어나려면 누군가가 `Future`를 실행해야 합니다.

```rust,edition2018
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:hello_world}}
```

`async fn` 블록 내부에서는 `.await`을 사용해서 다른 `async fn`의 결과같이 `Future` 트레잇을 구현한 다른 타입이 완료되기를 기다릴 수 있습니다. `block_on`과 달리 `.await`는 현재 스레드를 차단하지 않습니다. 대신 future가 지금 실행될 수 없다면 future가 완료될 때까지 비동기적으로 기다리면서 다른 작업을 실행합니다.

예를 들어 `learn_song`, `sing_song`, `dance`라는 3개의 `async fn`이 있다고 생각해봅시다.

```rust,ignore
async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }
```

노래를 배우고, 노래를 부르고, 춤을 추는 방법 중에는 각각을 실행하는 동안 다른 행동을 차단하는 방법도 있습니다.

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

하지만 우리는 이런 식으로 가능한 최고의 성능을 뽑을 수 없습니다. 한 번에 하나씩만 하고 있으니까요. 간단히 말해서 노래하기 전에 노래를 배우긴 해야겠지만 노래하면서 동시에 춤을 출 수도 있습니다. 이렇게 하려면 동시에 실행할 수 있는 두 개의 `async fn`을 만들어야 합니다.

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

여기서 노래를 부르기 전에 노래를 배워야 하지만 노래를 배우고 부르는 것 모두 춤을 추면서 할 수 있습니다. `learn_and_sing`의 `learn_song().await` 대신 `block_on(learn_song())`을 사용하면 `learn_song`이 실행되는 동안 스레드가 다른 작업을 할 수 없습니다. 그러면 동시에 춤을 추는 것이 불가능한 거죠. `.await`으로 `learn_song` future를 기다리게 하면 `learn_song`이 블락되었을 때 다른 작업이 현재의 스레드를 사용하게 할 수 있습니다. 이를 통해 동일한 스레드에서 완료해야 하는 여러 future를 동시에 실행할 수 있습니다.
