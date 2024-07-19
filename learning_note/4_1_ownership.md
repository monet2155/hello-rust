## 소유권

- 소유권은 러스트 프로그램의 메모리 관리법을 지배하는 규칙 모음
- 컴파일러가 컴파일 중에 검사할 여러 규칙을 기반으로 메모리를 관리 (체크리스트같은 느낌)
- 규칙 중 하나라도 위반하면 컴파일되지 않음
- 주요 목표는 힙 데이터의 관리

### 소유권 규칙

- 각각의 값은 소유자(owner)가 정해져 있다.
- 한 값의 소유자는 동시에 여럿일 수 없다.
- 소유자가 스코프 밖으로 벗어날 때, 값은 버려진다(dropped).

### 변수의 스코프

- 스코프란 아이템이 유효한 범위

```rust
{                      // s는 아직 선언되지 않아서 여기서는 유효하지 않습니다
    let s = "hello";   // 이 지점부터 s가 유효합니다

    // s로 어떤 작업을 합니다
}                      // 이 스코프가 종료되었고, s가 더 이상 유효하지 않습니다
```

### String 타입

- String 타입은 힙에 저장되기 때문에 Rust의 데이터 정리 과정을 알아보는데 적합함.
- 여태까지는 문자열을 하드코딩하는 문자열 리터럴을 사용했지만, 문자열 리터럴은 불변성을 지니기에 변경할 수 없음.

String 타입은 아래와 같이 `from` 함수와 문자열 리터럴을 이용해 생성 가능

```rust
let s = String::from("Hello");
```

아래 String 문자열은 변경이 가능함 (위는 immutable)

```rust
let mut s = String::from("hello");
s.push_str(", world!");
```

위 코드를 보았을때 String 또한 문자열 리터럴을 사용하여 생성하는데, 문자열 리터럴과 String은 어떤 차이가 있기에 변경할 수 있거나 없는지?

=> 각 타입의 메모리 사용 방식에 있음.

### 메모리와 할당

- 문자열 리터럴은 컴파일 타임에 내용을 알 수 있으므로, 최종 실행파일에 하드코딩 됨.
- 반면 String 타입은 힙에 메모리를 할당하기 때문에 텍스트 내용, 크기를 변경할 수 있는 것임. 반면 이 특징은 다음과 같은 현상을 유발함.
  - 실행 도중 메모리 할당자에게 메모리 할당을 요청해야함
  - String 사용을 마친 후 메모리를 해제할 방법이 필요함

첫 번째는 `String::from`을 호출 할 때 이미 해결됨

중요한 것은 두 번째, Rust에서는 이 문제를 스코프를 벗어나는 순간 자동으로 해제하도록 해결함.

```rust
{
    let mut s = String::from("hello");
    s.push_str(", world!");
}   // s 는 여기서 해제됨
```

Rust는 변수가 스코프 밖으로 벗어날 때 `drop` 이라는 특별한 함수를 호출함

`drop` 함수는 커스텀 타입을 개발한 개발자가 직접 메모리 해제 코드를 작성할 수 있게 함 (C++ 소멸자같은 개념)

### 이동 (Move)

```rust
let x = 5;
let y = x;
```

위 코드는 알다시피 y와 x가 모두 5라는 값을 가진다(값 복사)

```rust
let s1 = String::from("hello");
let s2 = s1;
```

String은 조금 다르게, ptr, len, capacity 세개의 그룹으로 이루어지며 이 그룹은 스택에 저장된다.

ptr이 가리키는 것은 힙 메모리의 문자열 시작부분('h')가 된다.

`s2 = s1;`를 실행하면 스택과 힙 메모리 모두 복사가 일어나는(값 복사) 것이 아닌, 스택의 그룹만 복사가 된다.

이는 `s2`와 `s1`이 가리키는(ptr) 힙 메모리 위치가 동일하다는 뜻이다. (참조 복사)

rust는 스코프를 벗어날 때 자동으로 `drop` 함수가 호출되며 해당 변수가 사용하는 메모리를 할당 해제하게 되는데, `s2`와 `s1` 모두 같은 힙 메모리를 가리키기에 중복 해제를 발생시키게 될 수 있다.

rust에서는 이를 방지하기 위해 한 가지 디테일이 추가되었는데,

할당된 메모리를 복사하는 대신, `s1`을 더이상 유효하지 않다고 간주하고, `s1`이 스코프 밖으로 벗어나더라도 해제하지 않는 것이다.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

따라서 위 코드에서는 `use of moved value`컴파일 에러가 발생한다.

이것이 Rust의 <strong>이동</strong> 이다.

### 클론 (Clone)

만약 String을 깊이 복사(Deep copy)하고 싶다면, `Clone`이라는 공용 메소드를 사용할 수 있다.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

위 코드를 사용하면 원하는 대로 힙 데이터가 복사된다.

### 스택의 복사

Rust는 정수형과 같이 스택에 저장할 수 있는 타입이 `Copy` trait이라고 불리는 어노테이션을 가진다.

어떤 타입이 `Copy` trait을 가진다면 대입하더라도 `이동`이 발생하지 않는다. (이전 변수를 계속 사용할 수 있다.)

어떤 타입이 `Drop` trait을 구현했다면 `Copy` trait을 어노테이션 할 수 없다.

`Copy`를 가질 수 있는 타입은 어떤게 있을까?

- `u32`와 같은 모든 정수형
- `bool`
- `f64`와 같은 모든 부동 소수점
- `Copy`가 가능한 타입만으로 구성된 `tuple`
  - `(i32, i32)` : 가능
  - `(i32, String)` : 불가능

### 소유권과 함수

함수에게 값을 넘기는(인자로 전달하는) 것은 변수에 값을 대입하는 것과 유사하다.
(대입과 마찬가지로 이동하거나 복사된다.)

```rust
fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다...
                                    // ... 그리고 이제 더이상 유효하지 않습니다.
    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
  // 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.
```

`s`는 `takes_ownership`으로 소유권이 <strong>이동</strong> 되어 함수 내부(스코프)에서 해제되고, 함수 호출 이후부터는 사용할 수 없다!!

### 반환 값과 스코프

값의 반환 또한 소유권을 이동시킨다.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
                                        // 이동시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
                                        // 이동되었고, 이 함수가 반환값을 s3으로도
                                        // 이동시켰습니다.

} // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로
  // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
  // 벗어나서 drop이 호출됩니다.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을
                                             // 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string                              // some_string이 반환되고, 호출한 쪽의
                                             // 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프
                                                      // 안으로 들어왔습니다.

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}
```

1. `gives_ownership`에서 `some_string`을 반환(스코프를 벗어남)해도 할당 해제되지 않고, `main`의 `s1`으로 소유권이 <strong>이동</strong>된다.
2. `s2`는 `takes_and_gives_back` 으로 소유권이 <strong>이동</strong>되었다가, `a_string`이 반환되며 다시 `main`의 `s3`로 이동된다.
3. `main`의 스코프를 벗어나며 `s1`과 `s3`가 `drop`되고, `s2`는 이동되었으므로 아무 일도 일어나지 않는다.

만약 함수에게 값을 넘겨주지만, 소유권을 넘기지 않으려면 어떻게 해야할까?

운 좋게도 Rust는 이를 위한 기능인 <strong>참조자(references)</strong>를 갖고 있다.

---

... 4_2_references 참고
