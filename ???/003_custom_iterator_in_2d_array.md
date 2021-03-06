# Custom Iterator in 2d-Array


## Introduction

다음과 같은 2d-array가 있다고 생각하자. <p>
``` rust
let arr = [[0; 8]; 8];
```
0으로 가득 채워진 8x8 배열이다. <p>
위 배열의 3번째 행의 원소를 순회하는 반복자(Iterator)는 매우 쉽게 만들 수 있다. <p>
``` rust
arr[3].iter();
```
iter() 메소드는 배열을 포함한 모든 슬라이스에 구현되어 있는 메소드로 불변참조 반복자(immutable iterator)를 생성한다.<p>
가변참조의 경우는 iter_mut(), 소유권 이전을 원할 경우 into_iter()을 대신 사용하면 된다.<p>
``` rust
arr[3].iter_mut();  // mutable
arr[3].into_iter(); // ownership moved
```
여기까지는 매우 쉽다.<p>
이 문서에서는 아래 단계를 통해 최종적으로 2차원 배열의 임의의 원소에서 시작하는 임의의 방향(직선방향)의 반복자를 만들 것이다.
1. [임의의 열을 순회하는 반복자](#1-임의의-열을-순회하는-반복자)
2. [(0, 0) 원소에서 시작하는 (i, i) 원소를 대각선으로 순회하는 반복자](#2-0-0-원소에서-시작하는-i-i-원소를-대각선으로-순회하는-반복자)
3. [임의의 원소에서 시작하는 임의의 방향의 반복자](#3-임의의-원소에서-시작하는-임의의-방향의-반복자)


## 1. 임의의 열을 순회하는 반복자

반복자를 만들기에 앞서서 구현하려는 반복자의 형식을 생각해보자.<p>
앞의 예제에서 iter() 메소드가 반환한 반복자와 같은 형식의 반복자를 만들 것이다.<p>
Iterator 트레이트를 구현한 구조체를 일반적으로 반복자라고 부르며, 반복자는 next() 메소드를 통해 값을 순회할 수 있다.<p>
아래 예시는 3번째 행의 원소를 순회하는 반복자의 값을 보여준다.<p>
``` rust
let arr = [[0; 4]; 4];

let mut it = arr[3].iter();

assert_eq!(it.next(), Some(&0));
assert_eq!(it.next(), Some(&0));
assert_eq!(it.next(), Some(&0));
assert_eq!(it.next(), Some(&0));

assert_eq!(it.next(), None);
assert_eq!(it.next(), None);
assert_eq!(it.next(), None);
```
위의 assert 매크로들은 panic 없이 정상 작동한다. <p>

위 예제는 간소화를 위해 4x4 배열을 사용하였고, 불변참조 반복자를 생성하는 iter() 메소드를 사용했다.<p>
보다시피 next() 메소드로 값을 순회하다 보면 네 번째 원소까지는 배열의 3번째 행의 원소인 0 들이 Option<i32>의 열거값 Some에 싸여있는 형태로 반환되는 것이 확인된다. <p>
또한 불변참조이기 때문에 0 앞에 &가 붙어있는 것까지 확인가능하다.<p>
하지만 배열의 Out-of-Bound에 해당하는 4번째 원소부터는 계속해서 next()를 호출하는 것은 가능하나 Option<i32>의 열거값 None이 반환되는 것이 확인된다.<p>
이를 통해, 배열을 순회하는 반복자의 형식을 다음과 같이 생각할 수 있다.<p>
- 배열의 범위 안에서는 Some(원소값)을 반환. (불변 참조일 경우는 Some(&원소값), 가변 참조일 경우는 Some(&mut 원소값))
- 배열의 범위 밖에서는 None을 반환.

그럼 이제 반복자의 형식을 알았으므로 본격적으로 임의의 열을 순회하는 반복자를 이 형식에 맞춰서 만들어보자. <p>
두 가지 방법으로 만들어 볼 것인데, 첫 번째 방법은 새로운 iterator를 구현하는 방법이고, 두 번째 방법은 표준 라이브러리의 Iterator 메소드와 클로져를 적극 활용하는 방법이다. <p>
<p>
첫 번째 방법으로 새로운 iterator를 구현해보자.<p>
그러나 할 수 없다. 할 수는 있는데 배열의 수명도 처리해야하고 이러저러한 부분에서 매우 힘들다.<p>
아래 코드는 배열의 iter()의 내부를 가져온 코드다.<p>

``` rust
impl<'a, T> Iter<'a, T> {
    #[inline]
    pub(super) fn new(slice: &'a [T]) -> Self {
        let ptr = slice.as_ptr();
        // SAFETY: Similar to `IterMut::new`.
        unsafe {
            assume(!ptr.is_null());

            let end = if mem::size_of::<T>() == 0 {
                (ptr as *const u8).wrapping_add(slice.len()) as *const T
            } else {
                ptr.add(slice.len())
            };

            Self { ptr: NonNull::new_unchecked(ptr as *mut T), end, _marker: PhantomData }
        }
    }
```

대충봐도 unsafe도 쓰이고 전혀 쉬운방법이 아니다. 그러므로 두번째 방법인 표준라이브러리의 메소드들과 클로져를 적극 활용하기로 하자. <p>
아래 코드는 작성해 본 2열을 순회하는 반복자다.<p>
``` rust
let arr = [[0; 4]; 4];
let mut col_2_iter = 
    (0..4)
    .map(|i| &arr[i][2]);

assert_eq!(col_2_iter.next(), Some(&0));
assert_eq!(col_2_iter.next(), Some(&0));
assert_eq!(col_2_iter.next(), Some(&0));
assert_eq!(col_2_iter.next(), Some(&0));

assert_eq!(col_2_iter.next(), None);
```
설명은 생략. <p>


## 2. (0, 0) 원소에서 시작하는 (i, i) 원소를 대각선으로 순회하는 반복자
대각선의 경우는 참조하는 원소의 위치가 row, col 모두 변경한다는 부분이 다른 점이다. 아래와 같이 만들면 된다. <p>

``` rust
let arr = [[0; 4]; 4];
let mut iter =
    (0..4)
    .zip(0..4)
    .map(|(i, j)| &arr[i][j]);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));

assert_eq!(iter.next(), None);
```


## 3. 임의의 원소에서 시작하는 임의의 방향의 반복자

이것을 구현하기에 앞서 (1, 2)에서 시작하는 아래 방향 반복자를 만들어보자. <p>
``` rust
let arr = [[0; 4]; 4];

let mut iter = 
    (1..4)
    .map(|i| &arr[i][2]);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));

assert_eq!(iter.next(), None);
```

아직 역방향을 안했구나!<p>
(3, 2)에서 위로 올라가는 반복자를 만들어보자.<p>
``` rust
let arr = [[0; 4]; 4];

let mut iter = 
    (0..=3).rev()
    .map(|i| &arr[i][2]);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));

assert_eq!(iter.next(), None);


```
rust에서 감소하는 range 표현이 없다는 게 아쉬운 부분이다. 언젠가 생기길 원한다. <p>


여기까지 했다면 뭘하든 쉬워 보일 것이다. <p>
이제 (3,2)에서 대각선 오른쪽-위 방향으로 향하는 반복자를 만들어보자. <p>
i는 감소, j는 증가하므로 아래와 같이 작성하면 된다.<p>
``` rust
let arr = [[0; 4]; 4];

let mut iter = 
    (0..=3).rev()
    .zip(2..4)
    .map(|(i, j)| &arr[i][j]);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&0));

assert_eq!(iter.next(), None);
```
여기서 주목할 점은 (0..=3) 의 길이는 4이고, (2..4)의 길이는 2 이지만, zip은 둘 중 최소 길이를 사용하여 구현하는 사람이 굳이 i를 위한 반복자를 (0..=1)로 해줄 필요가 없다는 점이다. <p>

## 진짜 마지막으로...

임의의 좌표 (y, x)가 주어지고, 어느 방향이 어느 방향인지 코드 작성단계에서 모를 경우를 작성해보자.<p>
이게 사실 이 문서의 주 목적이었다. <p>
방향은 상수 배열로 8방향에 대한 y, x 값 증감을 미리 저장하는 형식으로 구현할 것이다. <p>
방향은 0 이 위를 가리키고, 0~7까지 시계 방향으로 할당한다.<p>

``` rust
const DY: [isize; 8] = [-1, -1, 0, 1, 1, 1, 0, -1];
const DX: [isize; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

fn main() {
    /* 배열의 모든 원소를 (i,j)로 초기화 */
    let mut arr: [[(usize, usize);4];4] = Default::default(); 
    (0..4).for_each(|i| (0..4).for_each(|j| arr[i][j] = (i, j)));
  
    /* 임의의 시작점과 방향 설정 */
    let (y, x, dir) = (1, 3, 5);

    /* 반복자 생성 */
    let mut iter =
        (0..)
        .map(|dist| (y + DY[dir] * dist, x + DX[dir] * dist))
        .take_while(|&(i, j)| i>=0 && i<4 && j>=0 && j<4)
        .map(|(i, j)| &arr[i as usize][j as usize]);
      
         
    assert_eq!(iter.next(), Some(&(1, 3)));
    assert_eq!(iter.next(), Some(&(2, 2)));
    assert_eq!(iter.next(), Some(&(3, 1)));

    assert_eq!(iter.next(), None);

}

```

여기서 주목할 점은 반복자의 lazy 성질을 활용하여 시작 반복자를 (0..)이라는 무한 반복자를 사용한 것이다. <p>
무한 반복자를 시작으로 map을 통해 좌표에 대한 반복자를 튜플로 뽑아내고, 범위 밖의 경우를 take_while로 걸러내었다. <p>
나머지는 rust의 단점이자 장점인 까다로운 타입체크인데, take_while이 input을 참조로 받아온다는 점과 배열의 인덱스는 항상 usize 타입이어야 한다는 점을 이용했다. <p>
이로써 2차원 배열 내의 가능한 직선 반복자를 모두 생성 가능해졌다. <p>
