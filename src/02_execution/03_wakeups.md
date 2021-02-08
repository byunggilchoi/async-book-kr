# task 깨우기와 `Waker`

future가 처음 `poll`했을 때 아직 완료되어 있지 않은 건 흔한 일입니다. 그러면 향후 더 변화가 있을 때 future가 다시 poll을 실행하도록 해야 합니다. `Waker` 타입이 이 역할을 수행합니다.

future가 poll을 실행할 때마다 "task"의 일부로 폴링됩니다. task는 실행자에게 제출되는 최상위 future입니다.

`Waker`는 실행자에게 관련된 task가 깨어나야 한다고 전해주는 `wake()` 메서드를 제공합니다. 그래서 `wake()`가 호출되면 실행자가 `Waker`와 연관된 task가 다음으로 넘어갈 준비가 되었다는 것과 task의 future가 다시 풀링되어야 한다는 것을 알게 됩니다.

`Waker`는 `clone()`을 구현하여 복사, 저장할 수 있습니다.

이제 `Waker`를 사용해서 간단한 future 타이머를 구현해보겠습니다.

## 응용: 타이머 만들기

예시를 만들어 봅시다. 타이머가 생성될 때 새 스레드를 만들고 수면 시간 동안은 자고 시간이 되면 타이머 future가 신호를 주는 걸로 하겠습니다.

시작하기 위해 필요한 라이브러리들은 다음과 같습니다.

```rust
{{#include ../../examples/02_03_timer/src/lib.rs:imports}}
```

future 타입 자체를 정의하는 것부터 시작하겠습니다. 이 future에는 스레드에게 타이머가 종료되고 future가 완료될 것이라는 것을 알릴 방법이 필요합니다. 그래서 스레드와 future 사이의 통신을 위해 `Arc<Mutex<..>>` 값을 공유해서 사용할 것입니다.

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:timer_decl}}
```

이제 실제 `Future`를 구현해봅시다.

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:future_for_timer}}
```

아주 간단하죠? 스레드가 `shared_state.completed = true`로 설정되어 있으면 완료된 것입니다! 그렇지 않으면 현재의 task에 대한 `Waker`를 복제해서 `shared_state.waker`에 넘겨줘서 스레드가 task 백업을 깨울 수 있게 하면 됩니다.

중요한 것은 future가 풀링될 때마다 `Waker`를 항상 업데이트해야 한다는 것입니다. 왜냐하면 future가 다른 task와 거기에 딸린 다른 `Waker`로 이동할 수 있기 때문입니다. 이런 일은 폴링된 후 task 사이에서 future가 전달될 때마다 일어납니다.

마지막으로 실제로 타이머를 구성하고 스레드를 시작할 API가 필요합니다.

```rust,ignore
{{#include ../../examples/02_03_timer/src/lib.rs:timer_new}}
```

우와! 이것이 간단한 타이머 future를 만드는데 필요한 전부입니다. 이 future를 실행할 집행자가 있기만 하다면요.
