The Rust Programming Language 日本語版
数当てゲームをプログラムする
https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html


use std::io;

fn main() {
    println!("Guess the number!");          # 数を当ててごらん

    println!("Please input your guess.");   # ほら、予想を入力してね

    let mut guess = String::new();



    # ::new行にある::という記法は、newがString型の関連関数であることを表しています。
    # 関連関数とは、String型の特定のオブジェクトよりも型(この場合はString)に対して 実装された関数のことであり、
    # 静的(スタティック)メソッドと呼ばれる言語もあります。

	# let mut guess = String::new();という行は、
	# 現在、新たに空のStringオブジェクトに束縛されている
	# 可変変数を作っているわけです。



    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");     # 行の読み込みに失敗しました



    # プログラムの冒頭でuse std::ioとしていなければ、
    # この関数呼び出しは、std::io::stdinと記述されていました。

    # stdin関数は、 std::io::Stdinオブジェクトを返し、この型は、
    # ターミナルの標準入力へのハンドルを表す型になります。

    # .read_line(&mut guess)は、標準入力ハンドルのread_line メソッドを呼び出して、
    # ユーザから入力を受け付けます。
    # また、read_lineメソッドに対して、&mut guessという引数を一つ渡していますね。

    # read_lineメソッドの仕事は、ユーザが標準入力したものすべてを取り出し、文字列に格納することなので、
    # 格納する文字列を引数として取ります。この文字列引数は、可変である必要があります。
    # メソッドがユーザ入力を追記して、文字列の中身を変えられるようにということですね。

    # &という記号は、この引数が参照であることを表し、これのおかげで、
    # データを複数回メモリにコピーせずとも、 コードの複数箇所で同じデータにアクセスできるようになるわけです。
    # 参照は複雑な機能であり、 とても安全かつ簡単に参照を使うことができることは、
    # Rustの主要な利点の一つでもあります。 そのような詳細を知らなくても、
    # このプログラムを完成させることはできます。 現時点では、変数のように、
    # 参照も標準で不変であることを知っておけばいいでしょう。
    # 故に、&guessと書くのではなく、&mut guessと書いて、可変にする必要があるのです。
    # (第4章で参照についてより詳細に説明します)

    # .foo()という記法で、メソッドを呼び出す時、改行と空白で長い行を分割する


    println!("You guessed: {}", guess);     #  次のように予想しました: {}
}

/*********************************************************************************************************************************************************************************************************/

* 所有権

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    println!("v[0] is: {}", v[0]); // エラー: v はムーブ済み    
}

変数の値はコピーではなくムーブとして扱われる。
一つのリソースの所有権はただ一つの変数に束縛される。

プリミティブ型はCopy Traitを持っているのでムーブではなくコピーされる。


* (不変な)参照と借用

fn print_all(v: Vec<u64>){
    for val in v {
        println!("{}",val);
    }
}

fn main(){
    let v = vec![1,2,3,4,5];
    print_all(v); // 問題なし
    print_all(v); // エラー: v はムーブ済み
}

引数として渡すときも、ムーブで渡されてしまう。

let v = vec![1,2,3,4,5];
for val in v { // 問題なし
    println!("{}",val);
}

for val in v { // エラー:vはムーブ済み
    println!("{}",val);
}

このシチュエーションを切り抜けるためには&(参照)をつける。

fn print_all(v: &Vec<u64>){
    for val in v {
        println!("{}",val);
    }
}

fn main(){
    let v = vec![1,2,3,4,5];
    // ここで参照をする。
    // 参照が許されるのは不変な場合のみなので値を変更してはいけない。
    // 表示するだけ。
    print_all(&v); // 問題なし　
    print_all(&v); // 問題なし
}

* 変更可能な参照

変更可能な参照を作るには&mutを使う。

変更可能な参照を通じて変数にアクセスするには、*が必要。

let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);

/*********************************************************************************************************************************************************************************************************/

* 構造体(struct)

Rustで構造体を定義するにはstructを使います。
structには、データを持たないUnit構造体を定義する場合、
フィールドに名前がないタプル構造体を定義する場合、
それ以外の通常の構造体を定義する場合の3種類の構文があります。

これら3種類の構造体をすべて使った例を下記に用意しました。

// struct 名前; （Unit構造体の構文）
struct Dummy;

// struct 名前(型, ..); （タプル構造体の構文）
struct Point(f64, f64);

// struct 名前 {フィールド: 型, ..} （通常の構造体の構文）
struct Color {
    r: u8,
    g: u8,
    // 最後のフィールドの末尾にもカンマを付けられる
    b: u8,
}

fn main() {
    // Unit構造体は名前でそのまま初期化
    let dummy = Dummy;

    // タプル構造体は関数のように初期化
    // 実際、関数として扱うこともできる
    let point = Point(0.0, 0.0);

    // タプル構造体のフィールドへのアクセス
    let x = point.0;

    // 普通の構造体の初期化
    let black = Color { r: 0, g: 0, b: 0};

    // 普通の構造体のフィールドへのアクセス
    let r = black.r;
}

/*********************************************************************************************************************************************************************************************************/



enum IpAddrKind {
    V4,
    V6,
}


Enumの値

以下のようにして、IpAddrKindの各列挙子のインスタンスは生成できます:


let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

enumの列挙子は、その識別子の元に名前空間分けされていることと、 2連コロンを使ってその二つを区別していることに注意してください。 
これが有効な理由は、こうすることで、値IpAddrKind::V4とIpAddrKind::V6という値は両方とも、 同じ型IpAddrKindになったからです。
そうしたら、例えば、どんなIpAddrKindを取る関数も定義できるようになります。

/*********************************************************************************************************************************************************************************************************/

C++ ムーブセマンティクスと右辺値の概念

シンタックスとセマンティクスについて

シンタックスはコードをどう書くか
セマンティクスはコードはどう動くか


コピーとムーブ

std::vector<int> twice_vector(std::vector<int>);  // 全要素値を2倍して返す関数

std::vector<int> v = { /*大量の要素*/ };
std::vector<int> w = twice_vector( v ); // 関数へ変数vの中身を渡したい
// これ以後は変数vを使わない

この変数vの中身は、関数twice_vectorへ渡した後はもう必要とされません。
なのに、関数呼び出し部分では必ずコピーが発生しています。

実行時コストが高くつく変数の中身のコピーを避けて、変数の中身を移動（ムーブ）してしまえば実行時コストを抑えられる、まさにそういうケースです。
つまり、ここではムーブを「最適化されたコピー」と考えています。
仮にここをコピーのままとしても、実行時コストが無駄にかかるだけで、プログラムの動作としては何ら影響がありません。

/*********************************************************************************************************************************************************************************************************/

* Box
Boxを使う場面は

use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    // このPointをヒープ上に割り当て、ポインタを返す。
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // (all the type annotations are superfluous)
    // Stack allocated variables
    // （以下では型を全て明示していますが、必須ではありません。）
    // この変数ははすべてスタック上に割り当てられる。
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    // Heap allocated rectangle
    // ヒープ上に割り当てられたRectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    // 関数の返り値をボックス化
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    // 間にもう一つポインタを挟む
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    // ボックスのサイズはポインタのサイズに等しい
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    // `boxed_point`の保持するデータを`unboxed_point`にコピーする
    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );
}


/*********************************************************************************************************************************************************************************************************/

* メモリ解放

Dropトレイトにはメソッドが一つだけしかありません。dropです。これは、オブジェクトがスコープから抜けた時に自動で呼ばれます。
Dropトレイトの主な使用目的は、インスタンスが所有する資源を開放することです。

Dropトレイトを実装している型の例としてはBox、Vec、String、File、Process等があげられます。
Dropトレイトは任意の型に対して手動で実装することができます。

以下の例ではdropメソッドにコンソールへの出力を追加することで、dropが呼ばれたタイミングが分かるようにしています。

struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to console.
// このちょっとした実装で、`drop`にコンソール出力機能がつきます。
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    // `drop`関数を用いて変数を手動で開放することもできます。
    drop(_a);
    // TODO ^ Try commenting this line
    // TODO ^ この行をコメントアウトしてみましょう。

    println!("end of the main function");

    // `_a` *won't* be `drop`ed again here, because it already has been
    // (manually) `drop`ed
    // `_a`はここで`drop`されることは *ない* 。なぜならば、上ですでに
    // （手動で）`drop`されているため。
}

/*********************************************************************************************************************************************************************************************************/
