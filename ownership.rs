* 所有権 * ownership

* 所有権規則

まず、所有権のルールについて見ていきましょう。 この規則を具体化する例を扱っていく間もこれらのルールを肝に銘じておいてください:

    Rustの各値は、所有者と呼ばれる変数と対応している。
    いかなる時も所有者は一つである。
    所有者がスコープから外れたら、値は破棄される。


* 変数スコープ

    * スタックに確保されるデータ型

    所有権の最初の例として、何らかの変数のスコープについて見ていきましょう。
    スコープとは、 要素が有効になるプログラム内の範囲のことです。

    この変数は、宣言された地点から、現在のスコープの終わりまで有効になります。
    次のコードには、スタックで積まれる変数sが有効な場所に関する注釈がコメントで付記されています。


    {                      // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello";   // sは、ここから有効になる

        // sで作業をする
    }                      // このスコープは終わり。もうsは有効ではない


    ここまでに重要な点は二つあります:

        sがスコープに入ると、有効になる
        スコープを抜けるまで、有効なまま


    * ヒープに確保されるデータ型

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // これは`hello, world!`と出力する

    この種の文字列は、可変化することができます:

    なぜ、String型は可変化できるのに、リテラルはできないのでしょうか？
    違いは、これら二つの型がメモリを扱う方法にあります。



* メモリと確保

文字列リテラルの場合、中身はコンパイル時に判明しているので、テキストは最終的なバイナリファイルに直接ハードコードされます。 
このため、文字列リテラルは、高速で効率的になるのです。
しかし、これらの特性は、 その文字列リテラルの不変性にのみ端を発するものです。
残念なことに、コンパイル時にサイズが不明だったり、 プログラム実行に合わせてサイズが可変なテキスト片用に一塊のメモリをバイナリに確保しておくことは不可能です。

String型では、可変かつ伸長可能なテキスト破片をサポートするために、コンパイル時には不明な量のメモリを ヒープに確保して内容を保持します。つまり:

    メモリは、実行時にOSに要求される。
    String型を使用し終わったら、OSにこのメモリを返還する方法が必要である。

この最初の部分は、既にしています: String::from関数を呼んだら、その実装が必要なメモリを要求するのです。 これは、プログラミング言語において、極めて普遍的です。

しかしながら、2番目の部分は異なります。ガベージコレクタ(GC)付きの言語では、GCがこれ以上、 使用されないメモリを検知して片付けるため、
プログラマはそのことを考慮する必要はありません。 GCがないなら、メモリがもう使用されないことを見計らって、
明示的に返還するコードを呼び出すのはプログラマの責任になります。ちょうど要求の際にしたようにですね。
これを正確にすることは、 歴史的にも難しいプログラミング問題の一つであり続けています。もし、忘れていたら、メモリを無駄にします。
タイミングが早すぎたら、無効な変数を作ってしまいます。2回解放してしまってもバグになるわけです。 allocateとfreeは完璧に1対1対応にしなければならないのです。

Rustは、異なる道を歩んでいます: ひとたび、メモリを所有している変数がスコープを抜けたら、 メモリは自動的に返還されます。
こちらの例は、 リスト4-1のスコープ例を文字列リテラルからString型を使うものに変更したバージョンになります:

{
    let s = String::from("hello"); // sはここから有効になる

    // sで作業をする
}                                  // このスコープはここでおしまい。sは
             

String型が必要とするメモリをOSに返還することが自然な地点があります: s変数がスコープを抜ける時です。 
変数がスコープを抜ける時、Rustは特別な関数を呼んでくれます。
この関数は、dropと呼ばれ、 ここにString型の書き手はメモリを返還するコードを配置することができます。Rustは、閉じ波括弧で自動的にdrop関数を呼び出します。



* 変数とデータの相互作用法: ムーブ

Rustにおいては、複数の変数が同じデータに対して異なる手段で相互作用することができます。 整数を使用したリスト4-2の例を見てみましょう。

let x = 5;
let y = x;
println!("{}", x); // エラーなし

もしかしたら、何をしているのか予想することができるでしょう:
「値5をxに束縛する; それからxの値をコピーしてyに束縛する。」これで、 二つの変数(xとy)が存在し、両方、値は5になりました。
これは確かに起こっている現象を説明しています。 なぜなら、整数は既知の固定サイズの単純な値で、これら二つの5という値は、スタックに積まれるからです。

では、String(ヒープ)バージョンを見ていきましょう:

let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // エラー: borrow of moved value: `s1`

このコードは先ほどのコードに酷似していますので、動作方法も同じだと思い込んでしまうかもしれません:
要するに、2行目でs1の値をコピーし、s2に束縛するということです。ところが、 これは全く起こることを言い当てていません。

s1をs2に代入すると、String型のデータがコピーされます。つまり、スタックにあるポインタ、長さ、 許容量をコピーするということです。
ポインタが指すヒープ上のデータはコピーしません。(実体はコピーされないということ)



* 変数とデータの相互作用法: クローン

仮に、スタック上のデータだけでなく、本当にString型のヒープデータのdeep copyが必要ならば、 cloneと呼ばれるよくあるメソッドを使うことができます。
メソッド記法については第5章で議論しますが、 メソッドは多くのプログラミング言語に見られる機能なので、以前に見かけたこともあるんじゃないでしょうか。

これは、cloneメソッドの動作例です:

let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);

これは問題なく動作し、図4-3で示した動作を明示的に生み出します。ここでは、 ヒープデータが実際にコピーされています。

cloneメソッドの呼び出しを見かけたら、何らかの任意のコードが実行され、その実行コストは高いと把握できます。 何か違うことが起こっているなと見た目でわかるわけです。


* スタックのみのデータ: コピー

まだ話題にしていない別の問題があります。 この整数を使用したコードは、一部をリスト4-2で示しましたが、うまく動作する有効なものです:

let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);

ですが、このコードは一見、今学んだことと矛盾しているように見えます: cloneメソッドの呼び出しがないのに、xは有効で、yにムーブされませんでした。

その理由は、整数のようなコンパイル時に既知のサイズを持つ型は、スタック上にすっぽり保持されるので、 実際の値をコピーするのも高速だからです。
これは、変数yを生成した後にもxを無効化したくなる理由がないことを意味します。 
換言すると、ここでは、shallow copyとdeep copyの違いがないことになり、
cloneメソッドを呼び出しても、一般的なshallow copy以上のことをしなくなり、 そのまま放置しておけるということです。

RustにはCopyトレイトと呼ばれる特別な注釈があり、 
整数のようなスタックに保持される型に対して配置することができます(トレイトについては第10章でもっと詳しく話します)。 
型がCopyトレイトに適合していれば、代入後も古い変数が使用可能になります。コンパイラは、 型やその一部分でもDropトレイトを実装している場合、
Copyトレイトによる注釈をさせてくれません。 型の値がスコープを外れた時に何か特別なことを起こす必要がある場合にCopy注釈を追加すると、
コンパイルエラーが出ます。 型にCopy注釈をつける方法について学ぶには、付録Cの「導出可能なトレイト」をご覧ください。

では、どの型がCopyなのでしょうか？ある型について、ドキュメントをチェックすればいいのですが、 一般規則として、単純なスカラー値の集合は何でもCopyであり、
メモリ確保が必要だったり、 何らかの形態のリソースだったりするものはCopyではありません。ここにCopyの型の一部を並べておきます。

    プリミティブ型。
    タプル。ただ、Copyの型だけを含む場合。例えば、(i32, i32)はCopyだが、 (i32, String)は違う。
    


* 所有権と関数

意味論的に、関数に値を渡すことと、値を変数に代入することは似ています。関数に変数を渡すと、 代入のようにムーブやコピーされます。
次のコードは変数がスコープに入ったり、 抜けたりする地点について注釈してある例です。

fn main() {
    let s = String::from("hello");  // sがスコープに入る

    takes_ownership(s);             // sの値が関数にムーブされ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // xがスコープに入る

    makes_copy(x);                  // xも関数にムーブされるが、
                                    // i32はCopyなので、この後にxを使っても
                                    // 大丈夫

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。
//

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
//

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。
//

takes_ownershipの呼び出し後にsを呼び出そうとすると、コンパイラは、コンパイルエラーを投げるでしょう。 これらの静的チェックにより、ミスを犯さないでいられます。



* 戻り値とスコープ

値を返すことでも、所有権は移動します。

fn main() {
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
    // ムーブする
    
    let s2 = String::from("hello");     // s2がスコープに入る
    
    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
    // 戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
// 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
    // 呼び出した関数にムーブする
    
    let some_string = String::from("hello"); // some_stringがスコープに入る
    
    some_string                              // some_stringが返され、呼び出し元関数に
    // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
    
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

変数の所有権は、毎回同じパターンを辿っています: 別の変数に値を代入するとムーブされます。 
ヒープにデータを含む変数がスコープを抜けると、データが別の変数に所有されるようムーブされていない限り、 dropにより片付けられるでしょう。

所有権を取り、またその所有権を戻す、ということを全ての関数でしていたら、ちょっとめんどくさいですね。
関数に値は使わせるものの所有権を取らないようにさせるにはどうするべきでしょうか。 
返したいと思うかもしれない関数本体で発生したあらゆるデータとともに、再利用したかったら、渡されたものをまた返さなきゃいけないのは、 非常に煩わしいことです。

タプルで、複数の値を返すことは可能です。

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}

でも、これでは、大袈裟すぎますし、ありふれているはずの概念に対して、作業量が多すぎます。 
私たちにとって幸運なことに、Rustにはこの概念に対する機能があり、参照と呼ばれます。



* 参照と借用

上記のタプルコードの問題は、String型を呼び出し元の関数に戻さないと、calculate_lengthを呼び出した後に、 Stringオブジェクトが使えなくなることであり、
これはStringオブジェクトがcalculate_lengthにムーブされてしまうためでした。

ここで、値の所有権をもらう代わりに引数としてオブジェクトへの参照を取るcalculate_length関数を定義し、 使う方法を見てみましょう:

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

まず、変数宣言と関数の戻り値にあったタプルコードは全てなくなったことに気付いてください。 
2番目に、&s1をcalcuate_lengthに渡し、その定義では、String型ではなく、&Stringを受け取っていることに注目してください。


これらのアンド記号が参照であり、これのおかげで所有権をもらうことなく値を参照することができるのです。 

let s1 = String::from("hello");

let len = calculate_length(&s1);

この&s1という記法により、s1の値を参照する参照を生成することができますが、これを所有することはありません。 
所有してないということは、指している値は、参照がスコープを抜けてもドロップされないということです。

同様に、関数のシグニチャでも、&を使用して引数sの型が参照であることを示しています。 説明的な注釈を加えてみましょう:

fn calculate_length(s: &String) -> usize { // sはStringへの参照
    s.len()
} // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
  // 何も起こらない

変数sが有効なスコープは、あらゆる関数の引数のものと同じですが、所有権はないので、sがスコープを抜けても、 参照が指しているものをドロップすることはありません。
関数が実際の値の代わりに参照を引数に取ると、 所有権をもらわないので、所有権を返す目的で値を返す必要はありません。

関数の引数に参照を取ることを借用と呼びます。現実生活のように、誰かが何かを所有していたら、 それを借りることができます。用が済んだら、返さなきゃいけないわけです。

そして、参照が許されるのは不変な場合のみに限られます。値を変更、追加、削除はできません。



* 可変な参照

変更可能な参照を作るには&mutを使います。

変更可能な参照を通じて変数にアクセスするには、*が必要です。

let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);


一捻り加えるだけでリスト4-6のコードのエラーは解決します:

ファイル名: src/main.rs

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

始めに、sをmutに変えなければなりませんでした。そして、&mut sで可変な参照を生成し、 some_string: &mut Stringで可変な参照を受け入れなければなりませんでした。

ところが、可変な参照には大きな制約が一つあります: 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てないことです。こちらのコードは失敗します:

let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

これがエラーです:

error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> src/main.rs:91:14
   |
   |     let r1 = &mut s; // 最初の可変な参照はここ
   |              ------ first mutable borrow occurs here
   |     let r2 = &mut s; // 二つ目の可変な参照はここ
   |              ^^^^^^ second mutable borrow occurs here
   |     println!("{}{}", r1, r2); // 最初の借用はここで終わり
   |                      -- first borrow later used here


この制約は、可変化を許可するものの、それを非常に統制の取れた形で行えます。これは新たなRustaceanにとっては壁です。
なぜなら、多くの言語では、いつでも好きな時に可変化できるからです。

この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点です。 データ競合とは、競合条件と類似していて、これら3つの振る舞いが起きる時に発生します:

    2つ以上のポインタが同じデータに同時にアクセスする。
    少なくとも一つのポインタがデータに書き込みを行っている。
    データへのアクセスを同期する機構が使用されていない。

データ競合は未定義の振る舞いを引き起こし、実行時に追いかけようとした時に特定し解決するのが難しい問題です。
しかし、Rustは、データ競合が起こるコードをコンパイルさえしないので、この問題が発生しないようにしてくれるわけです。

いつものように、波かっこを使って新しいスコープを生成し、同時並行なものでなく、複数の可変な参照を作ることができます。

let mut s = String::from("hello");

{
    let r1 = &mut s;
    
} // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる

let r2 = &mut s;

可変と不変な参照を組み合わせることに関しても、似たような規則が存在しています。このコードはエラーになります:

let mut s = String::from("hello");

let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！

これがエラーです:

error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:95:14
   |
   |     let r1 = &s; // 問題なし
   |              -- immutable borrow occurs here
   |     let r2 = &s; // 問題なし
   |     let r3 = &mut s; // 大問題！
   |              ^^^^^^ mutable borrow occurs here
   |     println!("{}{}", r1, r2);
   |                      -- immutable borrow later used here


ふう！さらに不変な参照をしている間は、可変な参照をすることはできません。不変参照の使用者は、 それ以降に値が突然変わることなんて予想してません！
しかしながら、複数の不変参照をすることは可能です。 データを読み込んでいるだけの人に、他人がデータを読み込むことに対して影響を与える能力はないからです。

これらのエラーは、時としてイライラするものではありますが、Rustコンパイラがバグの可能性を早期に指摘してくれ(それも実行時ではなくコンパイル時に)、
問題の発生箇所をズバリ示してくれるのだと覚えておいてください。そうして想定通りにデータが変わらない理由を追いかける必要がなくなります。



* 宙に浮いた参照

ポインタのある言語では、誤ってダングリングポインタを生成してしまいやすいです。
ダングリングポインタとは、 他人に渡されてしまった可能性のあるメモリを指すポインタのことであり、その箇所へのポインタを保持している間に、
メモリを解放してしまうことで発生します。対照的にRustでは、コンパイラが、 参照がダングリング参照に絶対ならないよう保証してくれます: 
つまり、何らかのデータへの参照があったら、 コンパイラは参照がスコープを抜けるまで、データがスコープを抜けることがないよう確認してくれるわけです。

ダングリング参照作りを試してみますが、コンパイラはこれをコンパイルエラーで阻止します:

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

こちらがエラーです:

  error[E0106]: missing lifetime specifier
  --> src/main.rs:93:16
   |
   | fn dangle() -> &String {
   |                ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime

このエラーメッセージは、まだ講義していない機能について触れています: ライフタイムです。 ライフタイムについては第10章で詳しく議論しますが、
ライフタイムに関する部分を無視すれば、 このメッセージは、確かにこのコードが問題になる理由に関する鍵を握っています:

this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.

dangleコードの各段階で一体何が起きているのかを詳しく見ていきましょう:

fn dangle() -> &String { // dangleはStringへの参照を返す

    let s = String::from("hello"); // sは新しいString

    &s // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 危険だ

sは、dangle内で生成されているので、dangleのコードが終わったら、sは解放されてしまいますが、 そこへの参照を返そうとしました。
つまり、この参照は無効なStringを指していると思われるのです。 よくないことです！コンパイラは、これを阻止してくれるのです。

ここでの解決策は、Stringを直接返すことです:

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

これは何の問題もなく動きます。所有権はムーブされ、何も解放されることはありません。



* 参照の規則

参照について議論したことを再確認しましょう:

    任意のタイミングで、一つの可変参照か不変な参照いくつでものどちらかを行える。
    参照は常に有効でなければならない。

次は、違う種類の参照を見ていきましょう: スライスです。


８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８８

// ! WORNING !ここから下はコピペしただけなので修正の必要があります。

* スライス型

所有権のない別のデータ型は、スライスです。スライスにより、コレクション全体というより、 その内の一連の要素を参照することができます。

ここに小さなプログラミング問題があります: 文字列を受け取って、その文字列中の最初の単語を返す関数を書いてください。 関数が文字列中に空白を見つけなかったら、文字列全体が一つの単語に違いないので、文字列全体が返されるべきです。

この関数のシグニチャについて考えてみましょう:

fn first_word(s: &String) -> ?

この関数、first_wordは引数に&Stringをとります。所有権はいらないので、これで十分です。 ですが、何を返すべきでしょうか？文字列の一部について語る方法が全くありません。しかし、 単語の終端の添え字を返すことができますね。リスト4-7に示したように、その方法を試してみましょう。

ファイル名: src/main.rs


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

リスト4-7: String引数へのバイト数で表された添え字を返すfirst_word関数

Stringの値を要素ごとに見て、空白かどうかを確かめる必要があるので、 as_bytesメソッドを使って、Stringオブジェクトをバイト配列に変換しています。

let bytes = s.as_bytes();

次に、そのバイト配列に対して、iterメソッドを使用してイテレータを生成しています:

for (i, &item) in bytes.iter().enumerate() {

イテレータについて詳しくは、第13章で議論します。今は、iterは、コレクション内の各要素を返すメソッドであること、 enumerateがiterの結果を包んで、代わりにタプルの一部として各要素を返すことを知っておいてください。 enumerateから返ってくるタプルの第1要素は、添え字であり、2番目の要素は、(コレクションの)要素への参照になります。 これは、手動で添え字を計算するよりも少しだけ便利です。

enumerateメソッドがタプルを返すので、Rustのあらゆる場所同様、パターンを使って、そのタプルを分配できます。 従って、forループ内で、タプルの添え字に対するiとタプルの1バイトに対応する&itemを含むパターンを指定しています。 .iter().enumerate()から要素への参照を取得するので、パターンに&を使っています。

forループ内で、バイトリテラル表記を使用して空白を表すバイトを検索しています。空白が見つかったら、その位置を返します。 それ以外の場合、s.len()を使って文字列の長さを返します。

    if item == b' ' {
        return i;
    }
}

s.len()

さて、文字列内の最初の単語の終端の添え字を見つけ出せるようになりましたが、問題があります。 usize型を単独で返していますが、これは&Stringの文脈でのみ意味を持つ数値です。 言い換えると、Stringから切り離された値なので、将来的にも有効である保証がないのです。 リスト4-7のfirst_word関数を使用するリスト4-8のプログラムを考えてください。

ファイル名: src/main.rs

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // wordの中身は、値5になる

    s.clear(); // Stringを空にする。つまり、""と等しくする

    // wordはまだ値5を保持しているが、もうこの値を有効に使用できる文字列は存在しない。
    // wordは完全に無効なのだ！
}

リスト4-8: first_word関数の呼び出し結果を保持し、Stringの中身を変更する

このプログラムは何のエラーもなくコンパイルが通り、wordをs.clear()の呼び出し後に使用しても、 コンパイルが通ります。wordはsの状態に全く関連づけられていないので、その中身はまだ値5のままです。 その値5を変数sに使用し、最初の単語を取り出そうとすることはできますが、これはバグでしょう。 というのも、sの中身は、5をwordに保存してから変わってしまったからです。

word内の添え字がsに格納されたデータと同期されなくなるのを心配することは、面倒ですし間違いになりやすいです！ これらの添え字の管理は、second_word関数を書いたら、さらに難しくなります。 そのシグニチャは以下のようになるはずです:

fn second_word(s: &String) -> (usize, usize) {

今、私たちは開始と終端の添え字を追うようになりました。特定の状態のデータから計算されたけど、 その状態に全く紐付かない値が増えました。いつの間にか変わってしまうので、同期を取る必要のある、関連性のない変数が3つになってしまいました。

運のいいことに、Rustにはこの問題への解決策が用意されています: 文字列スライスです。
文字列スライス

文字列スライスとは、Stringの一部への参照で、こんな見た目をしています:


let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

これは、String全体への参照を取ることに似ていますが、余計な[0..5]という部分が付いています。 String全体への参照というよりも、Stringの一部への参照です。開始..終点という記法は、開始から始まり、 終点未満までずっと続く範囲です。

[starting_index..ending_index]と指定することで、角かっこに範囲を使い、スライスを生成できます。 ここで、starting_indexはスライスの最初の位置、ending_indexはスライスの終端位置よりも、 1大きくなります。内部的には、スライスデータ構造は、開始地点とスライスの長さを保持しており、 スライスの長さはending_indexからstarting_indexを引いたものに対応します。以上より、 let world = &s[6..11];の場合には、worldはsの7バイト目へのポインタと5という長さを保持するスライスになるでしょう。

図4-6は、これを図解しています。

Rustの..という範囲記法で、最初の番号(ゼロ)から始めたければ、2連ピリオドの前に値を書かなければいいのです。 換言すれば、これらは等価です:


let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];

同様の意味で、Stringの最後のバイトをスライスが含むのならば、末尾の数値を書かなければいいのです。 つまり、これらは等価になります:


let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];

さらに、両方の値を省略すると、文字列全体のスライスを得られます。故に、これらは等価です:


let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

    注釈: 文字列スライスの範囲添え字は、有効なUTF-8文字境界に置かなければなりません。 マルチバイト文字の真ん中で文字列スライスを生成しようとしたら、エラーでプログラムは落ちるでしょう。 文字列スライスを導入する目的で、この節ではASCIIのみを想定しています; UTF-8に関するより徹底した議論は、 第8章の「文字列でUTF-8エンコードされたテキストを格納する」節で行います。

これら全ての情報を心に留めて、first_wordを書き直してスライスを返すようにしましょう。 文字列スライスを意味する型は、&strと記述します:

ファイル名: src/main.rs


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

リスト4-7で取った手段と同じ方法で単語の終端添え字を取得しています。つまり、最初の空白を探すことです。 空白を発見したら、文字列の最初を開始地点、空白の添え字を終了地点として使用して文字列スライスを返しています。

これで、first_wordを呼び出すと、元のデータに紐付けられた単独の値を得られるようになりました。 この値は、スライスの開始地点への参照とスライス中の要素数から構成されています。

second_word関数についても、スライスを返すことでうまくいくでしょう:

fn second_word(s: &String) -> &str {

これで、ずっと混乱しにくい素直なAPIになりました。なぜなら、Stringへの参照が有効なままであることをコンパイラが、 保証してくれるからです。最初の単語の終端添え字を得た時に、 文字列を空っぽにして先ほどの添え字が無効になってしまったリスト4-8のプログラムのバグを覚えていますか？ そのコードは、論理的に正しくないのですが、即座にエラーにはなりませんでした。問題は後になってから発生し、 それは空の文字列に対して、最初の単語の添え字を使用し続けようとした時でした。スライスならこんなバグはあり得ず、 コードに問題があるなら、もっと迅速に判明します。スライスバージョンのfirst_wordを使用すると、 コンパイルエラーが発生します:

ファイル名: src/main.rs

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!    (エラー！)

    println!("the first word is: {}", word);
}

こちらがコンパイルエラーです:

$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
(エラー: 不変として借用されているので、`s`を可変で借用できません)
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here (不変借用はここで発生しています)
17 | 
18 |     s.clear(); // error!        (エラー！)
   |     ^^^^^^^^^ mutable borrow occurs here (可変借用はここで発生しています)
19 | 
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here
                                                (不変借用はその後ここで使われています)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership`.

To learn more, run the command again with --verbose.

借用規則から、何かへの不変な参照がある時、さらに可変な参照を得ることはできないことを思い出してください。 clearはStringを切り詰める必要があるので、可変な参照を得ようとして失敗しているわけです。 RustのおかげでAPIが使いやすくなるだけでなく、ある種のエラー全てを完全にコンパイル時に排除してくれるのです！
文字列リテラルはスライスである

文字列は、バイナリに埋め込まれると話したことを思い出してください。今やスライスのことを知ったので、 文字列リテラルを正しく理解することができます。


let s = "Hello, world!";

ここでのsの型は、&strです: バイナリのその特定の位置を指すスライスです。 これは、文字列が不変である理由にもなっています。要するに、&strは不変な参照なのです。
引数としての文字列スライス

リテラルやString値のスライスを得ることができると知ると、first_wordに対して、もう一つ改善点を見出すことができます。 シグニチャです:

fn first_word(s: &String) -> &str {

もっと経験を積んだRustaceanなら、代わりにリスト4-9のようなシグニチャを書くでしょう。というのも、こうすると、 同じ関数をString値と&str値両方に使えるようになるからです。

fn first_word(s: &str) -> &str {

リスト4-9: s引数の型に文字列スライスを使用してfirst_word関数を改善する

もし、文字列スライスがあるなら、それを直接渡せます。Stringがあるなら、 そのString全体のスライスを渡せます。Stringへの参照の代わりに文字列スライスを取るよう関数を定義すると、 何も機能を失うことなくAPIをより一般的で有益なものにできるのです。

Filename: src/main.rs

fn main() {
    let my_string = String::from("hello world");

    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word(&my_string_literal[..]);

    // 文字列リテラルは、すでに文字列スライス*な*ので、
    // スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);
}

他のスライス

文字列リテラルは、ご想像通り、文字列に特化したものです。ですが、もっと一般的なスライス型も存在します。 この配列を考えてください:


let a = [1, 2, 3, 4, 5];

文字列の一部を参照したくなる可能性があるのと同様、配列の一部を参照したくなる可能性もあります。 以下のようにすれば、参照することができます:


let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

このスライスは、&[i32]という型になります。これも文字列スライスと同じように動作します。 つまり、最初の要素への参照と長さを保持することです。他のすべての種類のコレクションに対して、 この種のスライスは使用するでしょう。これらのコレクションについて詳しくは、 第8章でベクタについて話すときに議論します。
まとめ

所有権、借用、スライスの概念は、コンパイル時にRustプログラムにおいて、メモリ安全性を保証します。 Rust言語も他のシステムプログラミング言語と同じように、メモリの使用法について制御させてくれるわけですが、 所有者がスコープを抜けたときにデータの所有者に自動的にデータを片付けさせることは、この制御を得るために、 余計なコードを書いてデバッグする必要がないことを意味します。

所有権は、Rustの他のいろんな部分が動作する方法に影響を与えるので、これ以降もこれらの概念についてさらに語っていく予定です。 第5章に移って、structでデータをグループ化することについて見ていきましょう。
