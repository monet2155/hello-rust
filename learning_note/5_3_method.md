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

---

### 더 많은 매개변수를 가진 메서드

`Rectangle` 구조체에 또 다른 메서드를 구현해보자. 새 메서드는 다른 `Rectangle` 인스턴스를 받고, 현재 인스턴스의 면적 내에 인수로 받은 다른 `Rectangle`이 완전히 들어갈 수 있는지 판단한다.

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

`can_hold` 메서드가 잘 구현되면, 위 코드는 아래와 같이 출력될 것이다.

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

배운 내용대로, `impl Rectangle` 블록 내에 선언하고, 메서드 명은 `can_hold`로 작성할 것이다.

매개변수 타입은 샘플 코드의 호출부를 보면 알 수 있는데, `Rectangle` 인스턴스의 불변 참조자를 전달했으니 `&Rectangle` 타입이 될 것으로 유추할 수 있다.

반환 타입은 `bool` 타입이 될 것이다.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

이제 원하던 출력 결과를 얻을 수 있고, `self` 이외의 여러 매개변수를 추가할 수 있음을 알게되었다.

### 연관 함수

`impl` 블록 내에 구현된 모든 함수를 연관 함수(associated function)이라고 부르는데, 이는 `impl` 키워드 뒤에 나오는 타입(현재까지는 `Rectangle`을 주로 다뤘다.)과 모두 연관된 함수이기 때문이다.

만약 동작하는데에 해당 타입의 인스턴스가 필요하지 않다면 `self`를 매개변수로 갖지 않는 (따라서 메서드가 아니게 된다.) 연관 함수를 정의할 수도 있다.

우리는 `String::from` 함수로 이미 연관 함수를 사용해봤다.

---

메서드가 아닌 연관 함수는 구조체의 새 인스턴스를 반환하는 생성자로 자주 활용된다.

이 함수들은 보통 `new`라는 이름을 가진다. (Rust에서는 키워드가 아니다. 그저 보편적으로 사용하는 생성자 이름이다.)

치수 하나를 매개변수로 받아 정사각형 `Rectangle`을 생성하는 `square` 연관 함수를 만들어보자.

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

`Self` 키워드는 `impl` 키워드 뒤의 타입의 별칭으로 자동으로 `Rectangle`이 된다.

연관 함수를 호출할 때는 `let sq = Rectangle::square(3);` 처럼 구조체 명에 `::` 구문을 붙여 호출한다.

### 여러 개의 `impl` 블록

각 구조체는 여러 개의 `impl` 블록을 가질 수 있다.

여러 `impl` 블록을 유용하게 사용하는 경우는 제네릭 타입, 트레이트 등으로 10장에서 학습하도록 하자.

### 정리

구조체를 사용하면 도메인에 의미있는 커스텀 타입을 만들 수 있다.

또한, 구조체를 사용함으로 서로 관련있는 데이터를 하나로 묶어 관리하거나, 데이터에 이름을 붙여 명확하게 만들 수 있다.

`impl` 블록 내부에는 타입과 관련된 연관 함수, 메서드를 정의하여 인스턴스가 가질 동작을 명시할 수 있다.
