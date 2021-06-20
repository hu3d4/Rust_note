*spawnで新規スレッドを生成する

新規スレッドを生成するには、thread::spawn関数を呼び出し、
新規スレッドで走らせたいコードを含むクロージャ(クロージャについては第13章で語りました)を渡します。
リスト16-1の例は、メインスレッドと新規スレッドからテキストを出力します:

ファイル名: src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

この関数では、新しいスレッドは、実行が終わったかどうかにかかわらず、メインスレッドが終了したら停止することに注意してください。
このプログラムからの出力は毎回少々異なる可能性がありますが、だいたい以下のような感じでしょう:

hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!

thread::sleepを呼び出すと、少々の間、スレッドの実行を止め、違うスレッドを走らせることができます。
スレッドはおそらく切り替わるでしょうが、保証はありません: OSがスレッドのスケジュールを行う方法によります。
この実行では、コード上では立ち上げられたスレッドのprint文が先に現れているのに、メインスレッドが先に出力しています。
また、立ち上げたスレッドにはiが9になるまで出力するよう指示しているのに、メインスレッドが終了する前の5までしか到達していません。

このコードを実行してメインスレッドの出力しか目の当たりにできなかったり、オーバーラップがなければ、
範囲の値を増やしてOSがスレッド切り替えを行う機会を増やしてみてください。



*joinハンドルで全スレッドの終了を待つ

リスト16-1のコードは、メインスレッドが終了するためにほとんどの場合、立ち上げたスレッドがすべて実行されないだけでなく、
立ち上げたスレッドが実行されるかどうかも保証できません。原因は、スレッドの実行順に保証がないからです。

thread::spawnの戻り値を変数に保存することで、立ち上げたスレッドが実行されなかったり、
完全には実行されなかったりする問題を修正することができます。thread::spawnの戻り値の型はJoinHandleです。
JoinHandleは、そのjoinメソッドを呼び出したときにスレッドの終了を待つ所有された値です。
リスト16-2は、リスト16-1で生成したスレッドのJoinHandleを使用し、joinを呼び出して、
mainが終了する前に、立ち上げたスレッドが確実に完了する方法を示しています:

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

ハンドルに対してjoinを呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックします。
スレッドをブロックするとは、そのスレッドが動いたり、終了したりすることを防ぐことです。
joinの呼び出しをメインスレッドのforループの後に配置したので、リスト16-2を実行すると、以下のように出力されるはずです:

hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!

2つのスレッドが代わる代わる実行されていますが、handle.join()呼び出しのためにメインスレッドは待機し、
立ち上げたスレッドが終了するまで終わりません。

ですが、代わりにhandle.join()をforループの前に移動したらどうなるのか確認しましょう。こんな感じに:

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

メインスレッドは、立ち上げたスレッドが終了するまで待ち、それからforループを実行するので、以下のように出力はもう混ざらないでしょう:

hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!

どこでjoinを呼ぶかといったほんの些細なことが、スレッドが同時に走るかどうかに影響することもあります。



*スレッドでmoveクロージャを使用する

moveクロージャは、thread::spawnとともによく使用されます。 あるスレッドのデータを別のスレッドで使用できるようになるからです。

第13章で、クロージャの引数リストの前にmoveキーワードを使用して、
クロージャに環境で使用している値の所有権を強制的に奪わせることができると述べました。
このテクニックは、あるスレッドから別のスレッドに値の所有権を移すために新しいスレッドを生成する際に特に有用です。

リスト16-1において、thread::spawnに渡したクロージャには引数がなかったことに注目してください: 立ち上げたスレッドのコードで
メインスレッドからのデータは何も使用していないのです。
立ち上げたスレッドでメインスレッドのデータを使用するには、立ち上げるスレッドのクロージャは、必要な値をキャプチャしなければなりません。
リスト16-3は、メインスレッドでベクタを生成し、立ち上げたスレッドで使用する試みを示しています。
しかしながら、すぐにわかるように、これはまだ動きません:

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        // こちらがベクタ: {:?}
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

クロージャはvを使用しているので、vをキャプチャし、クロージャの環境の一部にしています。
thread::spawnはこのクロージャを新しいスレッドで走らせるので、その新しいスレッド内でvにアクセスできるはずです。
しかし、このコードをコンパイルすると、以下のようなエラーが出ます:

error[E0373]: closure may outlive the current function, but it borrows `v`,
which is owned by the current function
(エラー: クロージャは現在の関数よりも長生きするかもしれませんが、現在の関数が所有している
`v`を借用しています)
 --> src/main.rs:6:32
  |
6 |     let handle = thread::spawn(|| {
  |                                ^^ may outlive borrowed value `v`
7 |         println!("Here's a vector: {:?}", v);
  |                                           - `v` is borrowed here
  |
help: to force the closure to take ownership of `v` (and any other referenced
variables), use the `move` keyword
(助言: `v`(や他の参照されている変数)の所有権をクロージャに奪わせるには、`move`キーワードを使用してください)
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^

Rustはvのキャプチャ方法を推論し、println!はvへの参照のみを必要とするので、クロージャは、vを借用しようとします。
ですが、問題があります: コンパイラには、立ち上げたスレッドがどのくらいの期間走るのかわからないので、
vへの参照が常に有効であるか把握できないのです。

リスト16-4は、vへの参照がより有効でなさそうな筋書きです:

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    // いや〜！
    drop(v); // oh no!

    handle.join().unwrap();
}

このコードを実行できてしまうなら、立ち上げたスレッドはまったく実行されることなく即座にバックグラウンドに置かれる可能性があります。
立ち上げたスレッドは内部にvへの参照を保持していますが、メインスレッドは、第15章で議論したdrop関数を使用して、即座にvをドロップしています。
そして、立ち上げたスレッドが実行を開始する時には、vはもう有効ではなく、参照も不正になるのです。あちゃー！

help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^

クロージャの前にmoveキーワードを付することで、コンパイラに値を借用すべきと推論させるのではなく、
クロージャに使用している値の所有権を強制的に奪わせます。リスト16-5に示したリスト16-3に対する変更は、コンパイルでき、意図通りに動きます:

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

moveクロージャを使用していたら、メインスレッドがdropを呼び出すリスト16-4のコードはどうなるのでしょうか？moveで解決するのでしょうか？
残念ながら、違います;
リスト16-4が試みていることは別の理由によりできないので、違うエラーが出ます。クロージャにmoveを付与したら、vをクロージャの環境にムーブするので、
最早メインスレッドでdropを呼び出すことは叶わなくなるでしょう。代わりにこのようなコンパイルエラーが出るでしょう:

error[E0382]: use of moved value: `v`
(エラー: ムーブされた値の使用: `v`)
  --> src/main.rs:10:10
   |
6  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
10 |     drop(v); // oh no!
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
   (注釈: `v`の型が`std::vec::Vec<i32>`のためムーブが起きました。この型は、`Copy`トレイトを実装していません

再三Rustの所有権規則が救ってくれました！リスト16-3のコードはエラーになりました。
コンパイラが一時的に保守的になり、スレッドに対してvを借用しただけだったからで、これは、メインスレッドは理論上、
立ち上げたスレッドの参照を不正化する可能性があることを意味します。
vの所有権を立ち上げたスレッドに移動するとコンパイラに指示することで、 メインスレッドはもうvを使用しないとコンパイラに保証しているのです。
リスト16-4も同様に変更したら、メインスレッドでvを使用しようとする際に所有権の規則に違反することになります。
moveキーワードにより、Rustの保守的な借用のデフォルトが上書きされるのです; 所有権の規則を侵害させてくれないのです。

スレッドとスレッドAPIの基礎知識を得たので、スレッドでできることを見ていきましょう。



*メッセージ受け渡しを使ってスレッド間でデータを転送する

人気度を増してきている安全な並行性を保証する一つのアプローチがメッセージ受け渡しで、
スレッドやアクターがデータを含むメッセージを相互に送り合うことでやり取りします。
こちらが、Go言語のドキュメンテーションのスローガンにある考えです:
「メモリを共有することでやり取りするな; 代わりにやり取りすることでメモリを共有しろ」

メッセージ送信並行性を達成するためにRustに存在する一つの主な道具は、
チャンネルで、Rustの標準ライブラリが実装を提供しているプログラミング概念です。
プログラミングのチャンネルは、水の流れのように考えることができます。小川とか川ですね。
アヒルのおもちゃやボートみたいなものを流れに置いたら、水路の終端まで下流に流れていきます。

プログラミングにおけるチャンネルは、2分割できます: 転送機と受信機です。転送機はアヒルのおもちゃを川に置く上流になり、
受信機は、アヒルのおもちゃが行き着く下流になります。
コードのある箇所が送信したいデータとともに転送機のメソッドを呼び出し、別の部分がメッセージが到着していないか受信側を調べます。
転送機と受信機のどちらかがドロップされると、チャンネルは閉じられたと言います。

ここで、1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、出力するプログラムに取り掛かります。
チャンネルを使用してスレッド間に単純な値を送り、機能の説明を行います。一旦、そのテクニックに慣れてしまえば、
チャンネルを使用してチャットシステムや、多くのスレッドが計算の一部を担い、
結果をまとめる1つのスレッドにその部分を送るようなシステムを実装できるでしょう。

まず、リスト16-6において、チャンネルを生成するものの、何もしません。
チャンネル越しにどんな型の値を送りたいのかコンパイラがわからないため、これはまだコンパイルできないことに注意してください。

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}

リスト16-6: チャンネルを生成し、2つの部品をtxとrxに代入する

mpsc::channel関数で新しいチャンネルを生成しています; mpscはmultiple producer, single consumerを表しています。
簡潔に言えば、Rustの標準ライブラリがチャンネルを実装している方法は、1つのチャンネルが値を生成する複数の送信側と、
その値を消費するたった1つの受信側を持つことができるということを意味します。
複数の小川が互いに合わさって1つの大きな川になるところを想像してください: どの小川を通っても、送られたものは最終的に1つの川に行き着きます。
今は、1つの生成器から始めますが、この例が動作するようになったら、複数の生成器を追加します。

mpsc::channel関数はタプルを返し、1つ目の要素は、送信側、2つ目の要素は受信側になります。
tx と rx という略称は、多くの分野で伝統的に転送機と受信機にそれぞれ使用されているので、変数をそのように名付けて、各終端を示します。
タプルを分配するパターンを伴うlet文を使用しています; let文でパターンを使用することと分配については、第18章で議論しましょう。
このようにlet文を使うと、mpsc::channelで返ってくるタプルの部品を抽出するのが便利になります。

立ち上げたスレッドがメインスレッドとやり取りするように、転送機を立ち上げたスレッドに移動し、
1文字列を送らせましょう。リスト16-7のようにですね。
川の上流にアヒルのおもちゃを置いたり、チャットのメッセージをあるスレッドから別のスレッドに送るみたいですね。

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

リスト16-7: txを立ち上げたスレッドに移動し、「やあ」を送る

今回も、thread::spawnを使用して新しいスレッドを生成し、それからmoveを使用して、
立ち上げたスレッドがtxを所有するようにクロージャにtxをムーブしています。
立ち上げたスレッドは、メッセージをチャンネルを通して送信できるように、チャンネルの送信側を所有する必要があります。

転送側には、送信したい値を取るsendメソッドがあります。sendメソッドはResult<T, E>型を返すので、既に受信側がドロップされ、
値を送信する場所がなければ、送信処理はエラーを返します。この例では、エラーの場合には、パニックするようにunwrapを呼び出しています。
ですが、実際のアプリケーションでは、ちゃんと扱うでしょう: 第9章に戻ってちゃんとしたエラー処理の方法を再確認してください。

リスト16-8において、メインスレッドのチャンネルの受信側から値を得ます。
アヒルのおもちゃを川の終端で水から回収したり、チャットメッセージを取得するみたいですね。

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    // 値は{}です
    println!("Got: {}", received);
}

リスト16-8: 「やあ」の値をメインスレッドで受け取り、出力する

チャンネルの受信側には有用なメソッドが2つあります: recvとtry_recvです。receiveの省略形であるrecvを使っています。
これは、メインスレッドの実行をブロックし、値がチャンネルを流れてくるまで待機します。
一旦値が送信されたら、recvはそれをResult<T, E>に含んで返します。チャンネルの送信側が閉じたら、recvはエラーを返し、もう値は来ないと通知します。

try_recvメソッドはブロックせず、代わりに即座にResult<T, E>を返します: メッセージがあったら、それを含むOk値、
今回は何もメッセージがなければ、Err値です。
メッセージを待つ間にこのスレッドにすることが他にあれば、try_recvは有用です: try_recvを頻繁に呼び出し、メッセージがあったら処理し、
それ以外の場合は、再度チェックするまでちょっとの間、他の作業をするループを書くことができるでしょう。

この例では、簡潔性のためにrecvを使用しました; メッセージを待つこと以外にメインスレッドがすべき作業はないので、
メインスレッドをブロックするのは適切です。

リスト16-8のコードを実行したら、メインスレッドから値が出力されるところを目撃するでしょう:

Got: hi

完璧です！



*チャンネルと所有権の転送

安全な並行コードを書く手助けをしてくれるので、所有権規則は、メッセージ送信で重要な役割を担っています。
並行プログラミングでエラーを回避することは、Rustプログラム全体で所有権について考える利点です。
実験をしてチャンネルと所有権がともに動いて、どう問題を回避するかをお見せしましょう: val値を立ち上げたスレッドで、
チャンネルに送った後に使用を試みます。リスト16-9のコードのコンパイルを試みて、このコードが許容されない理由を確認してください:

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // valは{}
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

リスト16-9: チャンネルに送信後にvalの使用を試みる

ここで、tx.send経由でチャンネルに送信後にvalを出力しようとしています。
これを許可するのは、悪い考えです: 一旦、値が他のスレッドに送信されたら、
再度値を使用しようとする前にそのスレッドが変更したりドロップできてしまいます。
可能性として、その別のスレッドの変更により、矛盾していたり存在しないデータのせいでエラーが発生したり、予期しない結果になるでしょう。
ですが、リスト16-9のコードのコンパイルを試みると、Rustはエラーを返します:

error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
not implement the `Copy` trait

並行性のミスがコンパイルエラーを招きました。send関数は引数の所有権を奪い、値がムーブされると、受信側が所有権を得るのです。
これにより、送信後に誤って再度値を使用するのを防いでくれます; 所有権システムが、万事問題ないことを確認してくれます。



*チャンネルと所有権の転送

安全な並行コードを書く手助けをしてくれるので、所有権規則は、メッセージ送信で重要な役割を担っています。
並行プログラミングでエラーを回避することは、Rustプログラム全体で所有権について考える利点です。
実験をしてチャンネルと所有権がともに動いて、どう問題を回避するかをお見せしましょう: val値を立ち上げたスレッドで、
チャンネルに送った後に使用を試みます。リスト16-9のコードのコンパイルを試みて、このコードが許容されない理由を確認してください:

use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // valは{}
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

リスト16-10: 複数のメッセージを送信し、メッセージ間で停止する

ここで、tx.send経由でチャンネルに送信後にvalを出力しようとしています。
これを許可するのは、悪い考えです: 一旦、値が他のスレッドに送信されたら、
再度値を使用しようとする前にそのスレッドが変更したりドロップできてしまいます。
可能性として、その別のスレッドの変更により、矛盾していたり存在しないデータのせいでエラーが発生したり、予期しない結果になるでしょう。
ですが、リスト16-9のコードのコンパイルを試みると、Rustはエラーを返します:

error[E0382]: use of moved value: `val`
  --> src/main.rs:10:31
   |
9  |         tx.send(val).unwrap();
   |                 --- value moved here
10 |         println!("val is {}", val);
   |                               ^^^ value used here after move
   |
   = note: move occurs because `val` has type `std::string::String`, which does
not implement the `Copy` trait

並行性のミスがコンパイルエラーを招きました。send関数は引数の所有権を奪い、値がムーブされると、受信側が所有権を得るのです。
これにより、送信後に誤って再度値を使用するのを防いでくれます; 所有権システムが、万事問題ないことを確認してくれます。



*複数の値を送信し、受信側が待機するのを確かめる

リスト16-8のコードはコンパイルでき、動きましたが、2つの個別のスレッドがお互いにチャンネル越しに会話していることは、明瞭に示されませんでした。
リスト16-10において、リスト16-8のコードが並行に動いていることを証明する変更を行いました: 立ち上げたスレッドは、複数のメッセージを送信し、
各メッセージ間で、1秒待機します。

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // スレッドからやあ(hi from the thread)
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

リスト16-10: 複数のメッセージを送信し、メッセージ間で停止する

今回は、メインスレッドに送信したい文字列のベクタを立ち上げたスレッドが持っています。
それらを繰り返し、各々個別に送信し、Durationの値1秒とともにthread::sleep関数を呼び出すことで、メッセージ間で停止します。

メインスレッドにおいて、最早recv関数を明示的に呼んではいません: 代わりに、rxをイテレータとして扱っています。
受信した値それぞれを出力します。 チャンネルが閉じられると、繰り返しも終わります。

リスト16-10のコードを走らせると、各行の間に1秒の待機をしつつ、以下のような出力を目の当たりにするはずです:

Got: hi
Got: from
Got: the
Got: thread

メインスレッドのforループには停止したり、遅れせたりするコードは何もないので、
メインスレッドが立ち上げたスレッドから値を受け取るのを待機していることがわかります。
ライフタイムとは、その参照が有効になるスコープのことです。多くの場合、型が推論されるように、大体の場合、ライフタイムも暗黙的に推論されます。



*転送機をクローンして複数の生成器を作成する

mpscは、mutiple producer, single consumerの頭字語であると前述しました。mpscを使い、リスト16-10のコードを拡張して、全ての値を同じ受信機に
送信する複数のスレッドを生成しましょう。チャンネルの転送の片割れをクローンすることでそうすることができます。リスト16-11のようにですね:

// --snip--

let (tx, rx) = mpsc::channel();

// txにtx1で受け取った値をコピーする
let tx1 = mpsc::Sender::clone(&tx);

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    // 君のためにもっとメッセージを(more messages for you)
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

// --snip--

リスト16-11: 複数の生成器から複数のメッセージを送信する

今回、最初のスレッドを立ち上げる前に、チャンネルの送信側に対してcloneを呼び出しています。
これにより、最初に立ち上げたスレッドに渡せる新しい送信ハンドルが得られます。元のチャンネルの送信側は、2番目に立ち上げたスレッドに渡します。
これにより2つスレッドが得られ、 それぞれチャンネルの受信側に異なるメッセージを送信します。

コードを実行すると、出力は以下のようなものになるはずです:

Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you

別の順番で値が出る可能性もあります; システム次第です。並行性が面白いと同時に難しい部分でもあります。
異なるスレッドで色々な値を与えてthread::sleepで実験をしたら、走らせるたびにより非決定的になり、毎回異なる出力をするでしょう。

チャンネルの動作方法を見たので、他の並行性に目を向けましょう。



*状態共有並行性

メッセージ受け渡しは、並行性を扱う素晴らしい方法ですが、唯一の方法ではありません。
Go言語ドキュメンテーションのスローガンのこの部分を再び考えてください: 「メモリを共有することでやり取りする。」

メモリを共有することでやり取りするとはどんな感じなのでしょうか？さらに、なぜメッセージ受け渡しに熱狂的な人は、
それを使わず、代わりに全く反対のことをするのでしょうか？

ある意味では、どんなプログラミング言語のチャンネルも単独の所有権に類似しています。
一旦チャンネルに値を転送したら、その値は最早使用することがないからです。
メモリ共有並行性は、複数の所有権に似ています: 複数のスレッドが同時に同じメモリ位置にアクセスできるのです。
第15章でスマートポインタが複数の所有権を可能にするのを目の当たりにしたように、異なる所有者を管理する必要があるので、
複数の所有権は複雑度を増させます。Rustの型システムと所有権規則は、この管理を正しく行う大きな助けになります。
例として、メモリ共有を行うより一般的な並行性の基本型の一つであるミューテックスを見てみましょう。



*ミューテックスを使用して一度に1つのスレッドからデータにアクセスすることを許可する

ミューテックスは、どんな時も1つのスレッドにしかなんらかのデータへのアクセスを許可しないというように、"mutual exclusion"(相互排他)の省略形です。
ミューテックスにあるデータにアクセスするには、ミューテックスのロックを所望することでアクセスしたいことをまず、
スレッドは通知しなければなりません。ロックとは、現在誰がデータへの排他的アクセスを行なっているかを追跡するミューテックスの
一部をなすデータ構造です。故に、ミューテックスはロックシステム経由で保持しているデータを死守する(guarding)と解説されます。

ミューテックスは、2つの規則を覚えておく必要があるため、難しいという評判があります:

    データを使用する前にロックの獲得を試みなければならない。
    ミューテックスが死守しているデータの使用が終わったら、他のスレッドがロックを獲得できるように、 データをアンロックしなければならない。

ミューテックスを現実世界の物で例えるなら、マイクが1つしかない会議のパネルディスカッションを思い浮かべてください。
パネリストが発言できる前に、マイクを使用したいと申し出たり、通知しなければなりません。マイクを受け取ったら、話したいだけ話し、
それから次に発言を申し出たパネリストにマイクを手渡します。パネリストが発言し終わった時に、マイクを手渡すのを忘れていたら、
誰も他の人は発言できません。共有されているマイクの管理がうまくいかなければ、パネルは予定通りに機能しないでしょう！

ミューテックスの管理は、正しく行うのに著しく技巧を要することがあるので、多くの人がチャンネルに熱狂的になるわけです。
しかしながら、Rustの型システムと所有権規則のおかげで、ロックとアンロックをおかしくすることはありません。



*Mutex<T>のAPI

ミューテックスの使用方法の例として、ミューテックスをシングルスレッドの文脈で使うことから始めましょう。 リスト16-12のようにですね:

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

リスト16-12: 簡潔性のためにMutex<T>のAPIをシングルスレッドの文脈で探究する

多くの型同様、newという関連関数を使用してMutex<T>を生成します。ミューテックス内部のデータにアクセスするには、
lockメソッドを使用してロックを獲得します。この呼び出しは、現在のスレッドをブロックするのでロックを得られる順番が来るまで何も作業はできません。

ロックを保持している他のスレッドがパニックしたら、lockの呼び出しは失敗するでしょう。その場合、誰もロックを取得することは叶わないので、
unwrapすると決定し、そのような状況になったら、このスレッドをパニックさせます。

ロックを獲得した後、今回の場合、numと名付けられていますが、戻り値を中に入っているデータへの可変参照として扱うことができます。
型システムにより、mの値を使用する前にロックを獲得していることが確認されます: Mutex<i32>はi32ではないので、i32を使用できるようにするには、
ロックを獲得しなければならないのです。忘れることはあり得ません; 型システムにより、それ以外の場合に内部のi32にアクセスすることは許されません。

お察しかもしれませんが、Mutex<T>はスマートポインタです。より正確を期すなら、lockの呼び出しがMutexGuardというスマートポインタを返却します。
このスマートポインタが、内部のデータを指すDerefを実装しています; このスマートポインタはさらにMutexGuardがスコープを外れた時に、
自動的にロックを解除するDrop実装もしていて、これがリスト16-12の内部スコープの終わりで発生します。
結果として、ロックの解除が自動的に行われるので、ロックの解除を忘れ、ミューテックスが他のスレッドで使用されるのを阻害するリスクを負いません。

ロックをドロップした後、ミューテックスの値を出力し、内部のi32の値を6に変更できたことが確かめられるのです。



*複数のスレッド間でMutex<T>を共有する

さて、Mutex<T>を使って複数のスレッド間で値を共有してみましょう。10個のスレッドを立ち上げ、各々カウンタの値を1ずつインクリメントさせるので、
カウンタは0から10まで上がります。以下の数例は、コンパイルエラーになることに注意し、そのエラーを使用してMutex<T>の使用法と、
コンパイラがそれを正しく活用する手助けをしてくれる方法について学びます。リスト16-13が最初の例です:

use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

リスト16-13: Mutex<T>により死守されているカウンタを10個のスレッドがそれぞれインクリメントする

リスト16-12のように、counter変数を生成してMutex<T>の内部にi32を保持しています。次に、
数値の範囲をマッピングして10個のスレッドを生成しています。thread::spawnを使用して、全スレッドに同じクロージャを与えています。
このクロージャは、スレッド内にカウンタをムーブし、lockメソッドを呼ぶことでMutex<T>のロックを獲得し、それからミューテックスの値に1を足します。
スレッドがクロージャを実行し終わったら、numはスコープ外に出てロックを解除するので、他のスレッドが獲得できるわけです。

メインスレッドで全てのjoinハンドルを収集します。それからリスト16-2のように、各々に対してjoinを呼び出し、
全スレッドが終了するのを確かめています。その時点で、メインスレッドはロックを獲得し、このプログラムの結果を出力します。

この例はコンパイルできないでしょうと仄めかしました。では、理由を探りましょう！

error[E0382]: capture of moved value: `counter`
(エラー: ムーブされた値をキャプチャしています: `counter`)
  --> src/main.rs:10:27
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
10 |             let mut num = counter.lock().unwrap();
   |                           ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:21:29
   |
9  |         let handle = thread::spawn(move || {
   |                                    ------- value moved (into closure) here
...
21 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors
(エラー: 前述の2つのエラーによりアボート)

エラーメッセージは、counter値はクロージャにムーブされ、それからlockを呼び出したときにキャプチャされていると述べています。
その説明は、所望した動作のように聞こえますが、許可されていないのです！

プログラムを単純化してこれを理解しましょう。forループで10個スレッドを生成する代わりに、
ループなしで2つのスレッドを作るだけにしてどうなるか確認しましょう。リスト16-13の最初のforループを代わりにこのコードと置き換えてください:

use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

2つのスレッドを生成し、2番目のスレッドの変数名をhandle2とnum2に変更しています。 今回このコードを走らせると、コンパイラは以下の出力をします:

error[E0382]: capture of moved value: `counter`
  --> src/main.rs:16:24
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
16 |         let mut num2 = counter.lock().unwrap();
   |                        ^^^^^^^ value captured here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error[E0382]: use of moved value: `counter`
  --> src/main.rs:26:29
   |
8  |     let handle = thread::spawn(move || {
   |                                ------- value moved (into closure) here
...
26 |     println!("Result: {}", *counter.lock().unwrap());
   |                             ^^^^^^^ value used here after move
   |
   = note: move occurs because `counter` has type `std::sync::Mutex<i32>`,
   which does not implement the `Copy` trait

error: aborting due to 2 previous errors

なるほど！最初のエラーメッセージは、handleに紐づけられたスレッドのクロージャにcounterがムーブされていることを示唆しています。
そのムーブにより、それに対してlockを呼び出し、結果を2番目のスレッドのnum2に保持しようとした時に、counterをキャプチャすることを妨げています！
ゆえに、コンパイラは、counterの所有権を複数のスレッドに移すことはできないと教えてくれています。これは、以前では確認しづらかったことです。
なぜなら、スレッドはループの中にあり、ループの違う繰り返しにある違うスレッドをコンパイラは指し示せないからです。
第15章で議論した複数所有権メソッドによりコンパイルエラーを修正しましょう。



*複数のスレッドで複数の所有権

第15章で、スマートポインタのRc<T>を使用して参照カウントの値を作ることで、1つの値に複数の所有者を与えました。
同じことをここでもして、どうなるか見ましょう。リスト16-14でRc<T>にMutex<T>を包含し、所有権をスレッドに移す前にRc<T>をクローンします。
今やエラーを確認したので、forループの使用に立ち戻り、クロージャにmoveキーワードを使用し続けます。

use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

リスト16-14: Rc<T>を使用して複数のスレッドにMutex<T>を所有させようとする

再三、コンパイルし……別のエラーが出ました！コンパイラはいろんなことを教えてくれています。

error[E0277]: the trait bound `std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send` is not satisfied in `[closure@src/main.rs:11:36:
15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
(エラー: トレイト境界`std::rc::Rc<std::sync::Mutex<i32>>:
std::marker::Send`は`[closure@src/main.rs:11:36:15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`で満たされていません)
  --> src/main.rs:11:22
   |
11 |         let handle = thread::spawn(move || {
   |                      ^^^^^^^^^^^^^ `std::rc::Rc<std::sync::Mutex<i32>>`
cannot be sent between threads safely
                          (`std::rc::Rc<std::sync::Mutex<i32>>`は、スレッド間で安全に送信できません)
   |
   = help: within `[closure@src/main.rs:11:36: 15:10
counter:std::rc::Rc<std::sync::Mutex<i32>>]`, the trait `std::marker::Send` is
not implemented for `std::rc::Rc<std::sync::Mutex<i32>>`
     (ヘルプ: `[closure@src/main.rs:11:36 15:10
     counter:std::rc::Rc<std::sync::Mutex<i32>>]`内でトレイト`std::marker::Send`は、
     `std::rc::Rc<std::sync::Mutex<i32>>`に対して実装されていません)
   = note: required because it appears within the type
`[closure@src/main.rs:11:36: 15:10 counter:std::rc::Rc<std::sync::Mutex<i32>>]`
     (注釈: 型`[closure@src/main.rs:11:36 15:10
     counter:std::rc::Rc<std::sync::Mutex<i32>>]`内に出現するので必要です)
   = note: required by `std::thread::spawn`
     (注釈: `std::thread::spawn`により必要とされています)

おお、このエラーメッセージはとても長ったらしいですね！こちらが、注目すべき重要な部分です: 最初のインラインエラーは
`std::rc::Rc<std::sync::Mutex<i32>>` cannot be sent between threads safelyと述べています。この理由は、
エラーメッセージの次に注目すべき重要な部分にあります。洗練されたエラーメッセージは、the trait bound `Send` is not satisfiedと述べています。
Sendについては、次の節で語ります。スレッドとともに使用している型が並行な場面で使われることを意図したものであることを保証するトレイトの1つです。

残念ながら、Rc<T>はスレッド間で共有するには安全ではないのです。Rc<T>が参照カウントを管理する際、cloneが呼び出されるたびにカウントを追加し、
クローンがドロップされるたびにカウントを差し引きます。しかし、
並行基本型を使用してカウントの変更が別のスレッドに妨害されないことを確認していないのです。これは間違ったカウントにつながる可能性があり、
今度はメモリリークや、使用し終わる前に値がドロップされることにつながる可能性のある潜在的なバグです。
必要なのは、いかにもRc<T>のようだけれども、参照カウントへの変更をスレッドセーフに行うものです。



*Arc<T>で原子的な参照カウント

幸いなことに、Arc<T>はRc<T>のような並行な状況で安全に使用できる型です。aはatomicを表し、原子的に参照カウントする型を意味します。
アトミックは、ここでは詳しく講義しない並行性の別の基本型です: 詳細は、std::sync::atomicの標準ライブラリドキュメンテーションを参照されたし。
現時点では、アトミックは、基本型のように動くけれども、スレッド間で共有しても安全なことだけ知っていれば良いです。

そうしたらあなたは、なぜ全ての基本型がアトミックでなく、
標準ライブラリの型も標準でArc<T>を使って実装されていないのか疑問に思う可能性があります。その理由は、
スレッド安全性が、本当に必要な時だけ支払いたいパフォーマンスの犠牲とともに得られるものだからです。
シングルスレッドで値に処理を施すだけなら、アトミックが提供する保証を強制する必要がない方がコードはより速く走るのです。

例に回帰しましょう: Arc<T>とRc<T>のAPIは同じなので、use行とnewの呼び出しとcloneの呼び出しを変更して、
プログラムを修正します。リスト16-15は、ようやくコンパイルでき、動作します:


use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

リスト16-15: Arc<T>を使用してMutex<T>をラップし、所有権を複数のスレッド間で共有できるようにする

このコードは、以下のように出力します:

Result: 10

やりました！0から10まで数え上げました。これは、あまり印象的ではないように思えるかもしれませんが、
本当にMutex<T>とスレッド安全性についていろんなことを教えてくれました。このプログラムの構造を使用して、
カウンタをインクリメントする以上の複雑な処理を行うこともできるでしょう。この手法を使えば、計算を独立した部分に小分けにし、
その部分をスレッドに分割し、それからMutex<T>を使用して、各スレッドに最終結果を更新させることができます。



*RefCell<T>/Rc<T>とMutex<T>/Arc<T>の類似性

counterは不変なのに、その内部にある値への可変参照を得ることができたことに気付いたでしょうか; つまり、Mutex<T>は、
Cell系のように内部可変性を提供するわけです。第15章でRefCell<T>を使用してRc<T>の内容を可変化できるようにしたのと同様に、
Mutex<T>を使用してArc<T>の内容を可変化しているのです。

気付いておくべき別の詳細は、Mutex<T>を使用する際にあらゆる種類のロジックエラーからは、コンパイラは保護してくれないということです。
第15章でRc<T>は、循環参照を生成してしまうリスクを伴い、そうすると、2つのRc<T>の値がお互いを参照し合い、
メモリリークを引き起こしてしまうことを思い出してください。同様に、Mutex<T>はデッドロックを生成するリスクを伴っています。
これは処理が2つのリソースをロックする必要があり、2つのスレッドがそれぞれにロックを1つ獲得して永久にお互いを待ちあってしまうときに起こります。
デッドロックに興味があるのなら、デッドロックのあるRustプログラムを組んでみてください; それからどんな言語でもいいので、
ミューテックスに対してデッドロックを緩和する方法を調べて、Rustで是非、それを実装してみてください。
Mutex<T>とMutexGuardに関する標準ライブラリのAPIドキュメンテーションは、 役に立つ情報を提供してくれます。

SendとSyncトレイトと、それらを独自の型で使用する方法について語って、この章を締めくくります。



*SyncとSendトレイトで拡張可能な並行性

面白いことに、Rust言語には、寡少な並行性機能があります。この章でここまでに語った並行性機能のほとんどは、
標準ライブラリの一部であり、言語ではありません。
並行性を扱う選択肢は、言語や標準ライブラリに制限されません; 独自の並行性機能を書いたり、他人が書いたものを利用したりできるのです。

ですが、2つの並行性概念が言語に埋め込まれています: std::markerトレイトのSyncとSendです。



*Sendでスレッド間の所有権の転送を許可する

Sendマーカートレイトは、Sendを実装した型の所有権をスレッド間で転送できることを示唆します。
Rustのほとんどの型はSendですが、Rc<T>を含めて一部例外があります: この型は、Rc<T>の値をクローンし、
クローンしたものの所有権を別のスレッドに転送しようとしたら、両方のスレッドが同時に参照カウントを更新できてしまうので、Sendになり得ません。
このため、Rc<T>はスレッド安全性のためのパフォーマンスの犠牲を支払わなくても済む、 シングルスレッド環境で使用するために実装されているわけです。

故に、Rustの型システムとトレイト境界により、Rc<T>の値を不安全にスレッド間で誤って送信することが絶対ないよう保証してくれるのです。
リスト16-14でこれを試みた時には、the trait Send is not implemented for Rc<Mutex<i32>>というエラーが出ました。
SendのArc<T>に切り替えたら、コードはコンパイルできたわけです。

完全にSendの型からなる型も全て自動的にSendと印付けされます。生ポインタを除くほとんどの基本型もSendで、生ポインタについては第19章で議論します。



*Syncで複数のスレッドからのアクセスを許可する

Syncマーカートレイトは、Syncを実装した型は、複数のスレッドから参照されても安全であることを示唆します。
言い換えると、&T(Tへの参照)がSendなら、型TはSyncであり、参照が他のスレッドに安全に送信できることを意味します。
Send同様、基本型はSyncであり、Syncの型からのみ構成される型もまたSyncです。

Sendではなかったのと同じ理由で、スマートポインタのRc<T>もまたSyncではありません。
RefCell<T>型(これについては第15章で話しました)と関連するCell<T>系についてもSyncではありません。
RefCell<T>が実行時に行う借用チェックの実装は、スレッド安全ではないのです。スマートポインタのMutex<T>はSyncで、
「複数のスレッド間でMutex<T>を共有する」節で見たように、複数のスレッドでアクセスを共有するのに使用することができます。



*SendとSyncを手動で実装するのは非安全である

SendとSyncトレイトから構成される型は自動的にSendとSyncにもなるので、それらのトレイトを手動で実装する必要はありません。
マーカートレイトとして、実装すべきメソッドさえも何もありません。並行性に関連する不変条件を強制することに役立つだけなのです。

これらのトレイトを手動で実装するには、unsafeなRustコードを実装することが関わってきます。
unsafeなRustコードを使用することについては第19章で語ります; とりあえず、重要な情報は、
SendとSyncではない部品からなる新しい並行な型を構成するには、安全性保証を保持するために、注意深い思考が必要になるということです。
The Rustonomiconには、これらの保証とそれを保持する方法についての情報がより多くあります。



*まとめ

この本において並行性を見かけるのは、これで最後ではありません: 第20章のプロジェクトでは、
この章の概念をここで議論した微小な例よりもより現実的な場面で使用するでしょう。

前述のように、Rustによる並行性の取扱いのごく一部のみが言語仕様なので、多くの並行性の解決策はクレートとして実装されています。
これらは標準ライブラリよりも迅速に進化するので、マルチスレッド環境で使用すべき現在の最先端のクレートを必ずネットで検索してください。

Rustの標準ライブラリは、メッセージ受け渡しにチャンネルを、並行の文脈で安全に使用できる、
Mutex<T>やArc<T>などのスマートポインタ型を提供しています。型システムと借用チェッカーにより、
これらの解決策を使用するコードがデータ競合や無効な参照に行き着かないことを保証してくれます。一旦コードをコンパイルすることができたら、
他の言語ではありふれている追跡困難な類のバグなしに、複数のスレッドでも喜んで動くので安心できます。
並行プログラミングは、もはや恐れるべき概念ではありません: 恐れることなく前進し、プログラムを並行にしてください！

次は、Rustプログラムが肥大化するにつれて問題をモデル化し、解決策を構造化する慣例的な方法について話します。
さらに、Rustのイディオムがオブジェクト指向プログラミングで馴染み深いかもしれないイディオムにどのように関連しているかについても議論します。
