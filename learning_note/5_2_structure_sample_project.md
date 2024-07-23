## 구조체를 사용한 예제 프로그램

어떨 때 구조체를 사용하면 좋을지 이해해 보기 위해 사각형 넓이를 계산하는 프로그램을 작성해보자.

... `structure_sample` 프로젝트 참고

### 트레이트 파생으로 유용한 기능 추가하기

프로그램을 디버깅하는 동안 `Rectangle` 인스턴스의 모든 필드 값을 출력해서 확인하면 좋을 것 같다.

```rust
// ...

let rect2 = Rectangle {
    width: 30,
    height: 50,
};
println!("rect2 is {}", rect2);
```

아쉽지만 위 코드는 작동하지 않는다.

`println!` 매크로에는 여러 출력 형식을 사용할 수 있는데, `{}`는 기본 형식으로 `Display`라는 출력 형식을 사용한다.

지금까지 사용했던 기본 타입들은 `Display`가 기본적으로 구현되어 있었다.

그러나 구조체는 `Display` 구현체가 기본제공되지 않는다.

위 코드의 에러 메시지를 확인해보면 `{}` 대신 `{:?}`를 사용해보는 것을 제안한다.

```rust
// ...
println!("rect2 is {:?}", rect1);
```

이는 `Display` 대신 `Debug` 라는 출력 형식을 사용하는 것이다.

그러나 이 또한 바로 적용되지는 않는데, 우리가 만든 구조체에 해당 기능을 적용하려면 명시적인 동의가 필요하다. 아래와 같이 외부 속성을 작성한다.

```rust
#[derive(Debug)] // 외부 속성 작성부
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect2 is {:?}", rect1);
}
```

이제 위 프로그램을 실행하면 더이상 에러가 발생하지 않고, 다음과 같은 출력이 나타난다.

```
...
rect2 is Rectangle { width: 30, height: 50 }
```

가장 예쁜 출력형태는 아니지만, 모든 필드값을 볼 수 있으니 확실히 유용해보인다.

필드가 더 많아진다면, `{:?}` 대신 `{:#?}`를 사용하면 아래처럼 출력되기에 보기 편하다.

```
....
rect 2 is Rectangle {
    width: 30,
    height: 50,
}
```

---

`Debug` 포맷을 사용하여 값을 출력하는 그 밖의 방법은 `dbg!` 매크로를 사용하는 것이다.

이 매크로는 표현식의 소유권을 가져와서, 값을 출력(`표준 에러 콘솔 스트림에 출력한다`)하고 다시 소유권을 반환한다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

위 코드는 아래와 같은 출력 화면을 보여준다.

```
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

`dbg!` 매크로를 호출한 파일과 해당 코드 라인, 값과 표현식 그 자체를 모두 보여주기 때문에 코드가 어떤 일을 하고 있는지 알아볼 때 매우 유용할 수 있다.

Rust에서는 이러한 `Debug` 트레이트 말고도 `derive` 속성으로 유용한 동작을 추가할 수 있는 여러 트레이트를 제공한다. (부록 C 참고)

또한, 이러한 트레이트를 직접 만들고, 커스터마이징 하는 방법 또한 10장에서 배울 예정이다.

---

현재 `area` 함수는 <strong>사각형</strong>의 면적만을 계산한다.

`Rectangle` 구조체를 제외한 다른 타입으로는 작동하지 않으니, `Rectangle` 구조체와 더 밀접하게 묶는 것이 좋아보인다.

다음 장에서 `Rectangle` 타입 안에 메서드 형태로 정의하여 리팩토링 하는 방법을 알아본다.
