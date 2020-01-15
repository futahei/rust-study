// 「ミュータビリティ」
// Rustにおけるミュータビリティの特徴として、デフォルトではないという点がある
let x = 5;
x = 6; // error!!

// mut キーワードでミュータビリティにできる
let mut x = 5;
x = 6; // no error.

// 上記の例では x の値を変更したのではなく、ある i32 から別の値へと束縛が変わったことを意味する
// 束縛が示す先を変更するには、ミュータブル参照が必要である
let mut x = 5;
let y = &mut x;
// y はミュータブル参照へのイミュータブルな束縛である。
// つまり y を他の束縛に変える(y = &mut z)ことはできないが、y に束縛されているものを
// 変化させることはできる(*y = 5)。

// 両方ミュータブルにしたいなら以下のようにする
let mut x = 5;
let mut y = &mut x;

// mut はパターンの一部を成すことに注意する
let (mut x, y) = (5, 6);
fn foo(mut x: i32) { }


// Rustで「イミュータブル」について言及する時、変更不可であることを意味せず、
// 「外側のミュータビリティ」を表す。例として以下を考える
use std::sync::Arc;

let x = Arc::new(5);
let y = x.clone();
// clone() を呼び出す時、Arc<T> は参照カウントを更新する必要がある。
// しかし、ここでは mut を一切使っておらず、x はイミュータブルな束縛であり、
// &mut 5 のような引数も取らない。

// これを理解するにはRustの持つ所有権システム、特に借用について立ち返る必要がある
// 「イミュータビリティ」の真の定義は二箇所以上から指されても安全であるということ
// Arc<T> の例では、変更に関して完全にそれ自身の構造の内側で行われるため、ユーザからは見えない
// なので clone() を用いて &T を配っている。仮に &mut T を配ってしまうと問題になる。

// 別の例では「内側のミュータビリティ」を考えることができる
use std::cell:RefCell;

let x = RefCell::new(42);
let y = x.borrow_mut();
// RefCellでは borrow_mut() メソッドにより、その内側にある値への &mut 参照を配る
// 実際これは危険であり、Rustの借用ルールを違反した場合 panic! を呼び出す


// ミュータビリティとは借用(&mut)や束縛(let mut)に関する属性、
// つまり一部がミュータブルで一部がイミュータブルなフィールドを持つ struct は作れない
struct Point {
  x: i32,
  mut y: i32, // これはできない
}

// 構造体へのミュータビリティは、それへの束縛の一部である
struct Point {
  x: i32,
  y: i32,
}

let mut a = Point { x: 5, y: 6 };

a.x = 10;

let b = Point { x: 5, y: 6 };

b.x = 10; // error: イミュータブルなフィールド b.x へ代入不可

// しかし、Cell<T> を使うことでフィールドレベルのミュータビリティが設定できるように振る舞える
use std::cell::Cell;

struct Point {
  x: i32,
  y: Cell<i32>,
}

let point = Point { x: 5, y: Cell::new(6) };

point.y.set(7);

println("y: {:?}", point.y); // y: Cell { value: 7 }

