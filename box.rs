* ヒープのデータを指すBox<T>を使用する

最も素直なスマートポインタはボックスであり、その型はBox<T>と記述されます。 ボックスにより、スタックではなくヒープにデータを格納することができます。
スタックに残るのは、 ヒープデータへのポインタです。スタックとヒープの違いを再確認するには、第4章を参照されたし。

ボックスは、データをスタックの代わりにヒープに格納する以外は、パフォーマンスのオーバーヘッドはありません。 しかし、多くのおまけの能力もありません。
以下のような場面で最もよく使用するでしょう:

    コンパイル時にはサイズを知ることができない型があり、正確なサイズを要求する文脈でその型の値を使用する時
    多くのデータがあり、所有権を転送したいが、その際にデータがコピーされないようにしたい時
    値を所有する必要があり、特定の型ではなく特定のトレイトを実装する型であることのみ気にかけている時

「ボックスで再帰的な型を可能にする」節で1つ目の場合について実際に説明します。 2番目の場合、多くのデータの所有権を転送するには、
データがスタック上でコピーされるので、長い時間がかかり得ます。 この場面でパフォーマンスを向上させるには、
多くのデータをヒープ上にボックスとして格納することができます。 そして、参照しているデータはヒープ上の1箇所に留まりつつ、
少量のポインタのデータのみをスタック上でコピーするのです。 3番目のケースは、トレイトオブジェクトとして知られ、
第17章の「トレイトオブジェクトで異なる型の値を許容する」の節は、 すべてその話題を説明するためだけのものです。 
従って、ここで学ぶのと同じことが第17章においても適用するでしょう！


＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊
* Box
Boxを使う場面は:

    コンパイル時にはサイズを知ることができない型があり、正確なサイズを要求する文脈でその型の値を使用する時
    多くのデータがあり、所有権を転送したいが、その際にデータがコピーされないようにしたい時
    値を所有する必要があり、特定の型ではなく特定のトレイトを実装する型であることのみ気にかけている時


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
    return Box::new(Point { x: 0.0, y: 0.0 })
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
