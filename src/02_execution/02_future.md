# `Future` 트레잇

`Future` 트레잇은 Rust 비동기 프로그래밍의 중심에 있습니다. `Future`는 (`()`와 같이 빈 값일 수도 있지만) 값을 생성할 수 있는 비동기 연산입니다. *단순화된* 버전의 future 트레잇은 아래와 같을 것입니다.

```rust
{{#include ../../examples/02_02_future_trait/src/lib.rs:simple_future}}
```

`poll` 함수를 호출해서 future를 다음 단계로 보낼 수 있는데 이는 future를 최대한 완료되도록 이끄는 역할을 합니다.(역자 주: "완료를 향해 이끈다"는 표현이 이해가 되지 않을 수 있습니다. 다음 문장에 나오지만, poll 메서드를 실행해도 결과가 나오지 않고 Ready(result) 혹은 Pending이라는, 가능한 두 상태를 담은 enum을 반환할 뿐입니다.) future가 완료되면 `Poll::Ready(result)`를 반환합니다. future가 아직 완료되지 않았으면 `Poll::Pending`을 반환한 후 `Future`가 다음 단계로 넘어갈 준비가 되었을 때 `wake()` 함수를 호출하도록 해놓습니다. `wake()`가 호출되면 실행자가 `Future`로 하여금 `poll`을 다시 호출하여 `Future`를 다음 단계로 넘어가도록 만듭니다.

`wake()`가 없었으면 실행자는 특정 future가 언제 다음 단계로 넘어갈 준비가 되는지 알 수가 없기 때문에 지속적으로 모든 future에서 폴링을 해야 합니다. `wake()`를 사용하면 실행자가 정확하게 어느 future가 `poll`을 실행할 준비가 되었는지 알 수가 있죠.

예를 들어 이미 사용한 데이터가 있거나 없을 수 있는 소켓에서 무언가를 읽고자 하는 경우를 생각해봅시다. 데이터가 있으면 읽어서 `Poll::Ready(data)`를 반환할 것이고 데이터가 준비되어 있지 않으면 future가 중단된 상태로 아무런 변화가 없을 것입니다. 아무 데이터도 사용할 수 없는 상태이면 데이터가 소켓에 준비되었을 때 호출할 `wake`를 등록해둬야 합니다. 간단한 `SocketRead` future는 아래와 같이 나오겠죠.

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:socket_read}}
```

이 `Future` 모델을 사용하면 중간 할당 없이도 여러 비동기 작업들을 함께 조합할 수 있습니다. 한 번에 여러 future들을 실행하거나 연결하는 것을 할당이 없는 상태 머신을 통해서도 다음과 같이 구현할 수 있죠.

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:join}}
```

이렇게 되면 별도의 할당없이도 여러 future들을 동시에 실행하여 보다 효율적인 비동기 프로그램이 가능합니다. 마찬가지로 다음과 같이 여러 future들을 순차적으로 실행할 수도 있습니다.

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:and_then}}
```

이 예제는 `Future` 트레잇을 사용하여 여러 개의 할당된 객체와 깊이 중첩된 콜백없이도 비동기 제어 흐름을 표현하는 방법을 보여줍니다. 이제 실제의 `Future` 트레잇을 보며 앞의 예제(역자주: SimpleFuture 트레잇)와의 차이점에 대해서 이야기해보겠습니다.

```rust,ignore
{{#include ../../examples/02_02_future_trait/src/lib.rs:real_future}}
```

첫번째 변경사항은 `self` 타입이 더 이상 `&mut Self`가 아니라 `Pin<&mut Self>`로 변경되었다는 것입니다. 피닝(pinning)에 대해서는 [다음 장][pinning]에서 더 자세하게 다룰 것이라서 지금은 피닝이 옮길 수 없는(move가 안 되는) future를 만들 수 있게 한다는 것 정도만 알고 있으면 됩니다. `struct MyFut { a: i32, ptr_to_a: *const i32 }`처럼 옮길 수 없는 객체는 필드에 포인터를 저장할 수 있습니다. async/await를 활성화하려면 피닝이 필요합니다.

두번째는 `wake: fn()`이 `&mut Context<'_>`로 변경되었습니다. 예제에서 다룬 `SimpleFuture`에서는 future 실행자에게 문제의 future에서 폴링해야 한다고 알리기 위해서 함수 포인터(`fn()`)를 호출했습니다. 그러나 `fn()`은 함수 포인터일 뿐이라서 어느 `Future`가 `wake`를 호출하는지에 대한 아무런 데이터도 저장할 수 없는 문제가 있습니다.

실제 시나리오에서 웹서버와 같은 복잡한 응용프로그램에는 '깨우기' 기능을 개별적으로 관리해야 하는 수천 개의 연결이 있을 수도 있습니다. 실제 `Future`트레잇에서 쓰는 `Context` 타입은 특정 작업을 깨우는데 사용할 수 있는 `Waker` 타입의 값을 저장해서 이 문제를 해결합니다.(역자주: `Context`타입에 `Waker`타입으로 어느 `Future`가 `wake`를 호출했는지 저장해둔다는 의미)

[pinning]: ../04_pinning/01_chapter.md
