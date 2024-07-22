## 슬라이스 (Slice)

슬라이스는 컬렉션을 통째로 참조하는 것이 아닌 컬렉션의 연속된 일련의 요소를 참조할 수 있게 해준다.

슬라이스는 참조자의 일종으로 소유권을 갖지 않는다.

---

단어들이 공백 문자로 구분된 문자열을 입력받고, 첫 번째 단어를 반환하는 함수를 작성해보자.

```rust
fn first_word(s: &String) -> ??
```

이 함수는 소유권을 가질 필요가 없으니(원본 데이터를 변경하지 않아도 된다.) 불변 참조자를 매개변수로 갖게 했다.

그런데 반환 타입이 지금으로써는 생각나지 않는다. 일단 공백 문자의 위치를 반환하도록 작성해보자.

```rust
fn first_word(s : &String) -> usize {
    let bytes = s.as_bytes();                       // String을 byte 배열로 변환한다.

    // 바이트 배열에 반복자를 생성하고, 튜플로 감싸 for loop (13장 참고)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' '는 바이트 리터럴 문법. ' '을 b(yte)로 바꾼다.
            return i; // 공백문자의 i(ndex)를 반환한다.
        }
    }

    s.len()
}
```

위 코드를 통해 이제 첫번째 단어 끝의 인덱스를 찾을 수 있게 되었다.

그런데 문제는 `usize`를 반환하고 있는데, 이 값이 향후에도 유효하다는(첫 번째 단어의 위치를 알고있는) 보장이 없다.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word는 값 5를 받습니다

    s.clear(); // 이 코드는 String을 비워서 ""으로 만듭니다

    // 여기서 word에는 여전히 5가 들어있지만, 이 5를 의미있게 쓸 수 있는
    // 문자열은 더 이상 없습니다. word는 이제 전혀 유효하지 않습니다!
}
```

위 코드는 정상적으로 컴파일 되고 작동도 잘 된다.

그러나 만약 위 코드 이후에 `word`(index를 담고있는)를 사용해서 `s`의 첫 단어를 추출하는데 사용할 경우 버그가 발생할 것이다.

심지어 두 번째 단어를 찾는 `second_word` 함수를 추가로 만든다면,

```rust
fn second_word(s: &String) -> (usize, usize) {
```

이러한 형태가 될텐데, 상당히 불편한 방법이다.

다행히도 Rust에서는 문자열 슬라이스라는 적절한 대안을 제공한다.

### 문자열 슬라이스

문자열 슬라이스는 String의 일부를 가리키는 참조자를 말한다.

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

만드는 방식은 String 참조자와 유사하며, `[starting_index..ending_index]`를 명시해 String의 일부분을 가리킨다. (ending_index에서 1을 뺀 위치까지 생성된다.)

`..` 범위 표현법은 0부터 시작할 경우 앞의 값을 생략할 수 있다. ex) `[..5]`

마찬가지로, 뒤의 값을 생략하면 맨 마지막 바이트까지 포함한다. ex) `[5..]`

앞 뒤 모두 생략할 경우 전체 문자열이 슬라이스로 생성된다. ex) `[..]`

---

이 문자열 슬라이스를 이용해 다시 `first_word` 함수를 작성해보자.

문자열 슬라이스를 나타내는 타입은 `&str`로 작성한다.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

이제 `first_word` 함수가 반환하는 값은 원래 데이터의 슬라이스를 반환한다.

위 `first_word` 함수를 사용했을 때 아래 코드는 이제 동작하지 않는다.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 에러!

    println!("the first word is: {}", word);
}
```

`clear` 함수는 String의 길이를 변경하기 때문에 가변 참조자가 필요하다.

그러나 `clear` 시점에는 이미 `s`에 대한 불변 참조자(`word`)가 존재하기 때문에, 이전에 학습한 내용과 충돌한다.

(아래 `println!`에서 `word`를 사용해야 하기때문에 `word`의 참조가 유효하게 유지된다.)

이렇게 Rust는 슬라이스와 같은 유용한 기능을 제공하고, 다양한 에러를 컴파일 타임에 제거한다.

### 슬라이스로써의 문자열 리터럴

앞서 문자열 리터럴 `"..."`이 바이너리에 저장된다고 학습했다.

```rust
let s = "Hello, world!";
```

여기서 `s`는 바이너리의 특정 지점을 가리키는 슬라이스(`&str` 타입)이다.

슬라이스는 불변 참조자이므로 문자열 리터럴을 변경할 수 없는 것이 자연스럽다.

### 문자열 슬라이스를 매개변수로 사용하기

리터럴과 String의 슬라이스를 만들 수 있게 되었으니, `first_word` 함수를 다음과 같이 정의할 수 있다.

```rust
fn first_word(s: &str) -> &str {...}
```

이제 문자열 슬라이스를 바로 인수로 전달할 수도 있고, String 원본이라면 String의 슬라이스 혹은 참조자를 전달할 수 있는 유연한 함수가 되었다.

이러한 유연성은 역참조 강제 변환(deref coercions) 기능을 이용하는데, 추후 15장에서 학습하도록 하자.

### 그 외 슬라이스

슬라이스는 문자열 뿐만 아니라 다른 타입의 컬렉션에도 범용적으로 사용할 수 있다. (8장 참고)

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

이 슬라이스는 `&[i32]` 타입으로, 동작 방식은 문자열 슬라이스와 동일하다.
