## 메서드 문법

메서드는 함수와 유사하다. `fn` 키워드로 선언하고, 매개변수와 반환 값을 가지며, 호출될 수 있다.

그러나 메서드는 함수와 달리 구조체 컨텍스트(또는 열거형이네 트레이트 객체 내부에)에 정의되며, 첫 번째 매개변수는 항상 `self`라는 차이점이 있다.

`self`는 메서드를 호출하고 있는 구조체 인스턴스를 나타낸다.

### 메서드 정의하기

이전 장의 `Rectangle` 매개변수를 갖던 `area` 함수를 수정하여 구조체 내부에 메서드를 정의해봅시다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

`Rectangle` 컨텍스트에 함수를 정의하기 위해 `impl` (implementation) 블록을 만드는 것으로 시작한다.

이 `impl` 내의 모든 것은 `Rectangle` 타입과 연관된다.

이제 `area` 함수의 매개변수를 `self`로, 본문 내용 또한 변경한다.

호출부에서는 메서드 문법으로 `Rectangle` 인스턴스의 `area` 메서드를 바로 호출할 수 있다.

---

`area` 선언부에서 매개변수의 타입을 `rectangle: &Rectangle` 대신 `&self`를 사용했다.

`&self`는 실제로는 `self : &Self`를 축약한 것으로, `impl` 블록 내에서 `Self`는 `impl` 블록의 대상이 되는 타입(위에서는 `Rectangle`)의 별칭이 된다.

---

구조체의 필드 이름과 동일한 이름의 메서드를 만들 수도 있다.

예를 들어, `width` 라는 이름의 메서드를 `Rectangle` 상에 정의할 수 있다.

`rect.width()` 와 같이 괄호를 붙이면 `width` 메서드를 의도한다는 것을 인지한다.

일반적으로 필드 이름과 동일한 이름의 메서드를 만드는 것은 해당 값을 얻어오는 용도(Getter)로 사용하는데, Rust는 Getter를 자동으로 생성하지 않기도 하고, 필드를 비공개로 하여 읽기 전용으로 만들 고 싶을때도 유용하다.

---

Rust에는 `->` 연산자가 존재하지 않는다.

C나 C++ 언어에서는 포인터를 통해 객체의 함수를 호출할때 `object->something()` 으로 사용한다.

Rust에는 이러한 기능을 하는 연산자는 존재하지 않는다. 자동 참조 및 역참고라는 기능이 존재하기 때문이다.

이 기능이 존재할 수 있는 이유는 메서드의 수신자(`self`의 타입)가 명확하기 때문이다.

수신자와 메서드명을 알면 해당 메서드가 인스턴스를 읽기만 하는지, 변경하는지, 소비하는지 알아낼 수 있기 때문이다.
