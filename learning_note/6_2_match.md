## match 제어 흐름 구조

Rust 는 `match`라고 불리는 매우 강력한 제어 흐름 연산자를 가진다.

이는 일련의 패턴에 대해 어떤 값이 매칭되었는지를 바탕으로 특정한 코드를 수행하도록 해준다.

패턴은 리터럴 값, 변수명, 와일드 카드 등 다앙한 것으로 구성될 수 있으며, 전체 종류와 역할은 18장에서 학습한다.

`match`의 장점은 패턴의 표현성에서 오며, 컴파일러는 모든 가능한 경우가 처리되는지 검사한다.

---

`match` 표현식을 동전 분류기와 비슷하게 볼 수 있다.

동전들은 다양한 크기의 구멍들이 있는 트랙으로 내려가고, 각 동전은 그 크기에 맞는 구멍을 만났을때 떨어진다.

동일한 방식으로, 값들은 `match` 내의 각 패턴을 통과하며 해당 값에 맞는 첫 번째 패턴에서 연관된 코드 블록으로 떨어진다.

동전 이야기가 나온 김에, `match` 를 이용한 예제로 동전 계수기 프로그램을 작성해보자.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

`match`를 영역별로 쪼개어 보자.

먼저, `match` 키워드 뒤에는 표현식이 온다. 위 코드의 경우 `coin` 이다. 이는 `if`에서 사용하는 조건식과 유사하지만, 가장 큰 차이는 조건문은 `bool` 값을 반환해야하며, 여기서는 어떤 타입이든 가능하다. (위 코드에서는 `coin`의 타입이므로 `Coin` 열거형이 된다.)

그다음은 `match`의 갈래(arm) 들이다. 하나의 갈래는 `패턴`과 `코드` 두 부분으로 이루어져있다.

위 코드에서 첫 번째 갈래에는 `Coin::Penny`라는 패턴이 있고, 그 뒤에 패턴과 코드를 구분하는 `=>` 연산자가 있다. 위 코드에서 `Coin::Penny`의 코드는 값 `1`이다.

각 갈래는 그 다음 갈래와 쉼표(,)로 구분되며, `match` 표현식이 실행될 때, 결과값을 각 갈래의 패턴에 대해서 순차적으로 비교한다.

각 갈래의 코드부는 표현식이며, 이 표현식의 결과는 `match` 표현식의 값이 된다.
(`Coin::Penny`일 경우, `value_in_cents`의 반환값이 `1`이 된다는 의미이다.)

각 갈래의 코드가 짧다면 위 코드와 같이 중괄호는 보통 사용하지 않는다. 만약 갈래 내에서 여러 줄의 코드가 필요하다면 중괄호를 사용하고, 이 때는 갈래 뒤에 쉼표를 붙이지 않아도 된다.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

### 값을 바인딩하는 패턴

매치 갈래의 또 다른 유용한 기능은 패턴과 매칭된 값의 일부분을 바인딩할 수 있다는 것이다.

예를 들어, 열거형의 배리언트 중 하나가 내부에 값을 들고있다고 해보자.

1999년부터 2008년까지 미국은 각 50개 주마다 한쪽 면의 디자인이 다른 쿼터 동전을 주조했다. 오직 쿼터 동전만이 주마다 디자인을 가진다. 이 정보를 `Quarter` 배리언트 내부에 `UsState` 값을 담도록 변경했다.

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

한 친구가 모든 50개 주의 커터 동전을 모으고 있다고 상상해보자.

동전 뭉치를 분류하는 동안 쿼터 동전과 연관된 주의 이름을 외치고, 만약 컬렉션에 없는 주의 동전이라면 컬렉션에 등록하여 따로 빼놓을 수도 있을 것이다.

이렇듯 `Quarter` 패턴에 매칭되었을 때, `주(UsState)`의 값을 가져오려면 아래와 같이 코드를 추가한다.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

위 코드에서 `Quarter` 배리언트의 내부 값은 `state`라는 값에 바인딩되어 중괄호 내부에서 사용할 수 있게 된다.

### `Option<T>`를 이용하는 매칭

이전 절에서 `Option<T>` 값을 사용하려면 `Some`일 때 실행되어 내부의 `T`값을 얻을 수 있는 코드가 필요하다고 언급했다.

이제 `Coin` 열거형을 다룬 것처럼 `Option<T>` 또한 `match`로 다룰 수 있을 것으로 생각된다.

`Option<i32>`를 매개변수로 받고, 내부에 값이 있으면 그 값에 1을 더하는 함수를 작성해보자.

값이 없다면 이 함수는 `None`을 반환하고 어떤 연산도 수행해서는 안된다.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
//...
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

위 코드의 함수 호출부를 자세히 들여다보자.

`plus_one(five)`가 호출될 때, `plus_one`의 `x`는 `Some(5)`를 갖게 된다.

`Some(5)`는 `None`과 같지 않으므로 다음 갈래로 넘어간다.

`Some(5)`는 `Some(i)`와 동일한 배리언트로, 패턴에 매칭된다.

`Some` 내부에 담긴 값(여기서는 `5`)는 `i`에 바인딩 되며, 코드부가 실행되어 `i`에 `1`을 더한 `6`을 담은 `Some`값을 반환한다.

이제 `x`가 `None`인 두 번째 호출을 살펴보자. `match` 안으로 들어와 첫번째 갈래에 매칭된다.

`match`는 종료되고, 코드부인 `None` 값을 반환한다.

`match`와 열거형을 조합하는 것은 다양한 경우에 유용하며, Rust 코드에서 이러한 패턴을 자주 보게 될 것이다.

---

### `match`는 철저하다.

`match` 갈래의 패턴들은 모든 가능한 경우를 다루어야 한다.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

위 코드는 컴파일조차 되지 않는다. `None` 패턴의 갈래를 작성하지 않았기 때문이다.

Rust의 `match`는 철저(exhaustive)하다. 유효한 코드를 만드려면 모든 가능성을 다루어야한다.

이러한 철저함은 이전에 학습한 널 개념 대신 `Option<T>`을 사용함에 따른 장점과도 일맥상통한다. 널인지 아닐지 모르는 값을 조작하는 실수를 불가능하게 만든다.

### 포괄 패턴과 \_ 자리 표시자

열거형을 사용하며 특정한 몇 개의 값들에만 동작을 수행하고, 그 외에는 기본 동작을 취하도록 작성할 수도 있다.

예를 들어, 주사위를 굴려 3이 나오면 모자를 얻고, 7이 나오면 모자를 잃으며, 이외의 값은 숫자만큼 칸을 이동하는 프로그램을 작성해보자.

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

처음 두 갈래의 패턴은 `3`과 `7` 리터럴 값이다.

나머지의 모든 값을 다루는 갈래의 패턴은 `other`라는 이름을 가진 변수로, 이 변수는 코드부로 연결되어 `move_player` 함수의 인자로 전달된다.

`match`가 모든 값을 나열하지 않았음에도 컴파일 되는 이유는, 나머지 모든 값에 대해 마지막 패턴이 매칭될 것이기 때문이다. 이러한 포괄 (catch-all) 패턴은 `match`의 철저함을 만족시킨다.

패턴들은 순차적으로 평가되므로, 마지막에 포괄적인 갈래를 위치시켜야 한다. (그렇지 않으면 그 뒤의 갈래는 실행되지 않으며, Rust는 경고를 준다.)

어쩌면 포괄 패턴이 필요하지만 그 값을 사용할 필요는 없을 수 있다. `_` 는 어떤 값이라도 매칭되지만 값을 바인딩하지는 않는 특별한 패턴이다. Rust는 이를 인지하고 사용되지 않는 변수에 대한 경고를 띄우지 않는다.

위 두 가지 방법을 이용해서 다양한 패턴을 적용해보자.

#### 3이나 7 이외의 값이 나오면 주사위를 다시 굴린다.

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

#### 3이나 7 이외의 값이 나오면 아무 일도 하지 않는다.

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

아무 일도 하지 않는 것은 유닛값(`()`)을 이용해서 명시적으로 Rust에게 알려주었다.

---

패턴과 매칭은 18장 학습을 통해 더 많이 배울 것이다.

다음 절에서는 `match` 표현식이 장황할 경우 유용한 `if let` 문법을 배운다.
