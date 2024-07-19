## 참조와 대여

소유권을 이동하는 대신 참조자(reference)를 만들 수 있다.

참조자는 해당 주소에 저장된 데이터에 접근할 수 있도록 해주는 주소값, 포인터와 같음.

참조자는 포인터와 달리 살아있는 동안 특정 타입에 대한 유효한 값을 가리킴을 보장한다.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calcculate_length(&s1)l

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

위 코드에서 `&` 기호가 참조자를 나타내고, 값을 참조할 수 있도록 한다.

`calculate_length` 함수의 `s`는 소유권이 없으므로 스코프 밖으로 나가지더라도 `s`의 `drop`은 발생하지 않는다.

그리고 참조자를 활용하는 행위를 대여(borrow) 라고 한다.

변수가 기본적으로 불변성을 지니듯, 참조자도 마찬가지로 가리키는 값을 수정할 수는 없다.

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // error!
}
```

### 가변 참조자

위 에러는 가변 참조자를 사용하는 식으로 없앨 수 있다.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

`s`를 `mut`로, `change`의 매개 변수 타입을 `&mut String`으로 변경한 후,

`change` 함수를 호출하는 곳에서 `&mut s`로 가변 참조자를 전달하면 된다.

---

가변 참조자는 한 가지 큰 제약 사항이 있는데, 어떤 값에 대한 가변 참조자가 있다면 해당 값에 대한 참조자는 더이상 만들 수 없다.

이 제약은 값의 변경에 대한 제어를 원활하게 해 주는데, 컴파일 타임에 데이터 경합(data race)을 방지할 수 있다.

데이터 경합이란 다음 세 가지 상황이 겹칠 때 발생하는 특정한 경합 조건(race condition)이다.

- 둘 이상의 포인터가 동시에 같은 데이터에 접근
- 포인터 중 하나 이상이 데이터에 쓰기 작업을 시행
- 데이터 접근 동기화 메커니즘이 없음

데이터 경합은 정의되지 않은 동작(undefined behavior)을 일으키며, 런타임에 추적하려고 할 때 문제 진단이 어렵다.

그러나 러스트에서는 데이터 경합이 발생할 수 있는 코드의 컴파일을 거부함으로 문제를 막을 수 있다.

---

가변 참조자 또는 불변 참조자와의 혼용이 가능한 상황을 알아보자.

```rust
let mut s = String::from("hello");
{
    let r1= &mut s;
}
let r2 = &mut s;
```

위 상황에서는 r1이 블록 스코프를 벗어날 때 가변 참조자가 해제되며, 문제 없이 새 참조자(`r2`)를 만들 수 있다.

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s; // 큰 문제!

println!("{}, {} and {}", r1, r2, r3);
```

위 상황에서는 불변 참조자가 있는 동안 같은 값의 가변 참조자가 생성되었기에 컴파일 에러가 발생한다.

불변 참조자를 사용하는 쪽에서는 값이 변경되리라 예상하지 않기 때문이다. (이 탓에 여러개의 불변 참조자는 생성이 가능하다.)

단, 참조자는 정의된 지점부터 마지막으로 사용된 부분까지 유효하다. 즉, 아래 코드는 컴파일 에러가 발생하지 않는다.

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
println!("{} and {}", r1, r2);
// 이 지점 이후부터 r1과 r2는 사용되지 않는다.

let r3 = &mut s;
println!("{}", r3);
```

위 코드는 컴파일러 수준에서 판단하기 때문에 동작한다.

### 댕글링 포인터 (Dangling pointer)

댕글링 포인터란 어떤 메모리를 가리키는 포인터가 남아있는 상황에서, 일부 메모리를 해제함으로 어떤 값을 가리키는지(다른 개체가 할당받았을 수 있는) 모르는 포인터를 의미한다.

Rust에서는 어떤 데이터의 참조자를 만들었을 때, 해당 참조자가 스코프를 벗어나기 전에 데이터가 먼저 벗어나는 지 컴파일러에서 확인하여 댕글링 포인터가 생성되지 않도록 보장한다.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

위 코드에서 발생하는 에러는 `missing lifetime specifier` 이며, lifetime에 대한 학습은 10장에서 진행한다.

에러 메시지의 핵심은 `이 함수는 빌린 값을 반환하고 있으나, 빌린 실제 값이 존재하지 않습니다.` 인데,

`dangle` 함수 스코프를 벗어날 때 `s`의 메모리는 `drop` 된다. 그리고 `s`의 참조자를 반환하기 때문에 댕글링 포인터가 발생하는데, Rust는 이를 막아주는 것이다. (에러 메시지의 핵심 그대로이다.)

따라서 이런 경우엔 String을 직접 반환하는 것이 적절하다. (소유권을 이동시킨다.)

### 참조자 규칙

정리하자면 다음과 같다.

- 단 하나의 가변 참조자를 갖거나, 여러 개의 불변 참조자를 가질 수 있다. (혼용은 불가능하다)
- 참조자는 항상 유효해야 한다.
