## 구조체

### 구조체 정의 및 인스턴트화

구조체는 여러 개의 연관된 값을 가질 수 있다.

튜플과 유사하게도, 구성 요소들이 각각 다른 타입이 될 수 있다.

구조체는 여기에 더해 각각의 구성 요소에 이름을 붙일 수 있어, 더 명확한 의미를 갖게되며 순서에 의존하지 않아도 된다.

구조체를 정의하려면 아래와 같이 작성한다.

```rust
struct User {
    // 구성요소들은 field 라고 부른다.
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

정의된 구조체를 사용하려면 각 필드에 대한 구체적인 값을 정하여 인스턴스(instance)를 생성해야한다.

인스턴스 생성은 아래와 같이 작성한다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

구조체의 특정 값에 접근하려면 점(.) 표기법을 사용한다. 값을 얻어오거나, 대입하는 것 또한 동일하다.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someon@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

인스턴스 변수를 가변성으로 지정하면 해당 인스턴스 전체가 가변성을 지니게 된다. 일부 필드만 가변으로 만들 수는 없다.

함수의 반환 값으로도 구조체 타입을 지정할 수 있으며, 인스턴스를 반환할 수 있다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

예제 코드에서 `username` 과 `email` 필드를 보면, 필드 이름과 대입하는 값의 변수명이 동일하여 약간 불편함이 발생한다.

이는 필드 초기화 축약법을 사용하여 개선할 수 있다.

### 필드 초기화 축약법 (field init shorthand)

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

(javascript와 동일하다.)

### 기존 인스턴스를 이용해 새 인스턴스를 만들 때 구조체 업데이트 문법 사용하기

다른 인스턴스에서 대부분의 값을 유지하고 특정 몇 개의 값만 바꿔 새로운 인스턴스를 생성하는 경우가 있다.

이 때 구조체 업데이트 문법을 사용하면 편리하다.

```rust
fn main() {
    // 생략

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    }
}
```

이렇게 하면 일일히 `username: user1.username` 등의 코드를 작성하지 않아도 되어 편리하다.

`..` 문법은 따로 명시된 필드를 제외한 나머지 필드를 주어진 인스턴스의 필드 값으로 설정한다.

또한 `..user1`은 가장 하단에 적어야 하며, 뒤에 콤마(,)가 붙지 않는다.

구조체 업데이트 문법은 대입처럼 `=` 연산을 이용한다. 이 연산은 이미 배운 것처럼 데이터를 이동시키며, 따라서 `user2` 가 생성된 이후부터 `user1`은 사용할 수 없게 된다.

단, `user2`를 생성하고 `user1`의 `active`와 `sign_in_count`만 사용한다면 `user1`은 유효하다. (`String` 값만 이동되었기 때문이다.)

### 명명된 필드 없는 튜플 구조체를 사용하여 다른 타입 만들기

Rust는 튜플과 유사한 튜플 구조체도 지원한다.

구조체 타입 자체에는 이름을 주어 의미를 주지만, 필드에는 이름을 붙이지 않고 타입만 적어 생성한다. (주로 튜플과 구분하고 싶지만 구조체는 너무 부담스러울 경우 사용한다.)

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

`black`과 `origin`의 필드 구성이 같더라도, 다른 구조체의 인스턴스이기 때문에 타입이 서로 다르다.

이를 제외하고는 튜플과 동일하게 사용할 수 잇다.

### 필드가 없는 유사 유닛 구조체

필드가 아예 없는 구조체를 정의할 수도 있다.

이는 튜플 타입을 공부할 때 배운 유닛 타입`()`과 유사하게 동작하므로 유사 유닛 구조체라고 지칭한다.

유사 유닛 구조체는 어떤 타입에 대해 트레이트를 구현하고 싶지만 타입 내부에 데이터를 저장할 필요는 없는 경우 유용하다.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

이 유사 유닛 구조체는 `AlwaysEqual`의 모든 인스턴스는 언제나 다른 모든 타입의 인스턴스와 같도록 하는 동작을 구현하여 테스트 용도로 사용할 수도 있을 것이다. (트레이트는 10장에서 학습 예정)

### 구조체 데이터의 소유권

위 `User` 구조체 정의해서 의도적으로 문자열 슬라이스(`&str`)이 아닌 String 타입을 사용했는데, 이는 구조체 인스턴스가 유효한 동안 각 인스턴스마다 데이터가 유효하길 기대하기 때문이다.

참조자를 이용하면 구조체가 소유권을 갖지않는 데이터도 저장할 수는 있지만, 이는 라이프타임(10장에서 학습 예정)을 활용해야 한다.

라이프 타임을 사용하면 구조체가 존재하는 동안에 구조체 내부의 참조자가 가리키는 데이터의 유효함을 보장받을 수 있다.

만약 라이프타임을 명시하지 않고 참조자를 저장하려고 하면 컴파일 에러가 발생한다.

(자세한 내용은 10장에서 학습하도록 한다.)
