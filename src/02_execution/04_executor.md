# 응용: 실행자 만들기

Rust의 `Future`는 게으릅니다. 적극적으로 완료로 이끌지 않는 이상 아무 것도 하지 않습니다. future를 완료하는 한 가지 방법은 `async` 함수 내에서 `.await`하는 것입니다. 하지만 이는 문제를 한 단계 미룰 뿐입니다. 결국 `async` 함수가 반환하는 future는 누가 실행할까요? 결국 `Future`는 실행자가 필요합니다.

`Future` 실행자는 최상위의 `Future` 집합을 가져와서 `Future`가 변화가 있을 때마다 `poll`을 호출해서 이들을 완료로 이끕니다. 일반적으로 실행자는 시작하기 위해 future에 대해 `poll`을 한 번 실행시킵니다. `Future`가 `wake()`를 호출하여 다음 단계로 넘어갈 준비를 마쳤음을 나타내면 큐에 다시 배치되고 또 `Future`가 완료될 때까지 `poll`을 반복해서 호출합니다.

이번 장에서는 대량의 최상위 future를 동시에 완료로 이끌어갈 간단한 실행자를 하나 만들어 볼 것입니다.

이번 예시에서는 `Waker`를 쉽게 만들 수 있는 `ArcWake` 트레잇을 가져오기 위해 `futures` 크레이트를 이용할 것입니다.

```toml
[package]
name = "xyz"
version = "0.1.0"
authors = ["XYZ Author"]
edition = "2018"

[dependencies]
futures = "0.3"
```

그 다음으로는 우리가 이용할 라이브러리 요소들을 `src/main.rs` 상단에 적어야 합니다.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:imports}}
```

실행자는 채널을 통해 실행할 task를 전송해서 작업합니다. 실행자는 채널에서 이벤트를 가져와서도 실행합니다. task가 더 많은 작업을 수행할 준비가 되면(즉 깨어나면) 스스로를 채널에 다시 배치하여 다시 폴링되도록 일정을 만들 수도 있습니다.

이 디자인에서 실행자 자체는 task 채널의 수신부만 있으면 됩니다. 사용자가 송신부를 가지고 새 future를 보내니까요. task 자체는 자신을 다시 일정에 걸 수 있는 future일 뿐이기 때문에 task를 자신을 다시 대기열에 추가할 때 사용할 수 있는 수신자와 짝을 지어서 future로 저장하도록 하겠습니다.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_decl}}
```

새로운 future를 쉽게 생성할 수 있도록 생성자(spawner)에도 메서드를 추가해보겠습니다. 이 메서드는 future 타입을 받아서 박스(box타입)에 넣고 실행자에 집어넣을 수 있는 새로운 `Arc<Task>`를 만듭니다.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:spawn_fn}}
```

future를 poll하려면 먼저 `Waker`를 만들어야 합니다. [task 깨우기 절]에서 논의했듯이 `Waker`는 `wake`가 호출되면 task가 다시 폴링되도록 일정에 등록합니다. `Waker`는 실행자에게 어떤 작업이 준비되었는지 정확히 알려주기 때문에 진행 준비가 된 future만 poll할 수 있습니다. 새로운 `Waker`를 만드는 가장 쉬운 방법은 `ArcWake` 트레잇을 구현하고 `waker_ref`나 `.into_waker()` 함수를 이용해서 `Arc<impl ArcWake>`를 `Waker`로 바꾸는 것입니다. task가 `Waker`가 되고 또 깨어날 수 있도록 `ArcWake`를 구현해 봅시다.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:arcwake_for_task}}
```

`Arc<Task>`에서 `Waker`가 만들어질 때 `wake()`를 호출하면 `Arc`의 사본이 task 채널로 전송됩니다. 그러면 실행자는 task를 선택해서 poll해야 합니다. 한 번 구현해 봅시다.

```rust,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:executor_run}}
```

축하합니다! 이제 우리는 잘 작동하는 future 실행자를 만들었습니다. `async/.await` 코드나 앞에서 만든 `TimeFuture`같은 

```rust,edition2018,ignore
{{#include ../../examples/02_04_executor/src/lib.rs:main}}
```

[task 깨우기 절]: ./03_wakeups.md
