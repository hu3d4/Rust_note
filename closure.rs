fn main() {
    use std::thread;
    use std::time::Duration;
    fn simulated_expensive_calculation(num: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }
    fn generate_workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!(
                // 今日は{}回腕立て伏せをしてください！
                "Today, do {} pushups!",
                simulated_expensive_calculation(intensity)
            );
            println!(
                // 次に、{}回腹筋をしてください！
                "Next, do {} situps!",
                simulated_expensive_calculation(intensity)
            );
        } else {
            if random_number == 3 {
                // 今日は休憩してください！水分補給を忘れずに！
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    // 今日は、{}分間走ってください！
                    "Today, run for {} minutes!",
                    simulated_expensive_calculation(intensity)
                );
            }
        }
    }
}

このコードは現在、ビジネスのほしいままに動くでしょうが、データサイエンスチームが、simulated_expensive_calculation関数を
呼び出す方法に何らかの変更を加える必要があると決定したとしましょう。そのような変更が起きた時に更新を簡略化するため、
simulated_expensive_calculation関数を1回だけ呼び出すように、このコードをリファクタリングしたいです。また、その過程でその関数への呼び出しを増やすことなく無駄に2回、
この関数を現時点で呼んでいるところを切り捨てたくもあります。要するに、結果が必要なければ関数を呼び出したくなく、それでも1回だけ呼び出したいのです。



*関数でリファクタリング
多くの方法でトレーニングプログラムを再構築することもできます。 1番目にsimulated_expensive_calculation関数への重複した呼び出しを変数に抽出しようとしましょう。
リスト13-4に示したように。

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

この変更によりsimulated_expensive_calculationの呼び出しが単一化され、最初のifブロックが無駄に関数を2回呼んでいた問題を解決します。
不幸なことに、これではあらゆる場合にこの関数を呼び出し、その結果を待つことになり、
結果値を全く使用しない内側のifブロックでもそうしてしまいます。

プログラムの1箇所でコードを定義したいですが、結果が本当に必要なところでだけコードを実行します。これは、クロージャのユースケースです！



*クロージャでリファクタリングして、コードを保存する

ifブロックの前にいつもsimulated_expensive_calculation関数を呼び出す代わりに、
クロージャを定義し、関数呼び出しの結果を保存するのではなく、そのクロージャを変数に保存できます。
リスト13-5のようにですね。simulated_expensive_calculationの本体全体を実際に、ここで導入しているクロージャ内に移すことができます。

let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

クロージャ定義が=に続き、変数expensive_closureに代入されています。
クロージャを定義するには、1組の縦棒から始め、その内部にクロージャの仮引数を指定します; この記法は、
SmalltalkやRubyのクロージャ定義と類似していることから選択されました。
このクロージャには、numという引数が1つあります: 2つ以上引数があるなら、 |param1, param2|のように、カンマで区切ります。

引数の後に、クロージャの本体を保持する波括弧を配置します(これはクロージャ本体が式一つなら省略可能です)。
波括弧の後、クロージャのお尻には、セミコロンが必要で、let文を完成させます。
クロージャ本体の最後の行から返る値(num)が、呼び出された時にクロージャから返る値になります。
その行がセミコロンで終わっていないからです; ちょうど関数の本体みたいですね。

このlet文は、expensive_closureが、匿名関数を呼び出した"結果の値"ではなく、匿名関数の"定義"を含むことを意味することに注意してください。
コードを定義して、1箇所で呼び出し、そのコードを保存し、後々、それを呼び出したいがためにクロージャを使用していることを思い出してください;
呼び出したいコードは、現在、expensive_closureに保存されています。

クロージャが定義されたので、ifブロックのコードを変更して、そのコードを実行するクロージャを呼び出し、結果値を得ることができます。
クロージャは、関数のように呼び出せます: クロージャ定義を含む変数名を指定し、使用したい引数値を含むかっこを続けます。
リスト13-6に示したようにですね。

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

今では、重い計算はたった1箇所でのみ呼び出され、その結果が必要なコードを実行するだけになりました。

ところが、リスト13-3の問題の一つを再浮上させてしまいました: それでも、最初のifブロックでクロージャを2回呼んでいて、
そうすると、重いコードを2回呼び出し、必要な分の2倍ユーザを待たせてしまいます。そのifブロックのみに属する変数を生成して、
クロージャの呼び出し結果を保持するそのifブロックに固有の変数を生成することでこの問題を解消することもできますが、
クロージャは他の解決法も用意してくれます。その解決策については、もう少し先で語りましょう。
でもまずは、クロージャ定義に型注釈がない理由とクロージャに関わるトレイトについて話しましょう。



*ジェネリック引数とFnトレイトを使用してクロージャを保存する

トレーニング生成アプリに戻りましょう。リスト13-6において、まだコードは必要以上の回数、重い計算のクロージャを呼んでいました。
この問題を解決する一つの選択肢は、重いクロージャの結果を再利用できるように変数に保存し、クロージャを再度呼ぶ代わりに、
結果が必要になる箇所それぞれでその変数を使用することです。しかしながら、この方法は同じコードを大量に繰り返す可能性があります。

運のいいことに、別の解決策もあります。クロージャやクロージャの呼び出し結果の値を保持する構造体を作れるのです。
結果の値が必要な場合のみにその構造体はクロージャを実行し、その結果の値をキャッシュするので、残りのコードは、結果を保存し、
再利用する責任を負わなくて済むのです。このパターンは、メモ化(memoization)または、遅延評価(lazy evaluation)として知っているかもしれません。

クロージャを保持する構造体を作成するために、クロージャの型を指定する必要があります。
構造体定義は、各フィールドの型を把握しておく必要がありますからね。各クロージャインスタンスには、
独自の匿名の型があります: つまり、たとえ2つのクロージャが全く同じシグニチャでも、その型はそれでも違うものと考えられるということです。
クロージャを使用する構造体、enum、関数引数を定義するには、第10章で議論したように、ジェネリクスとトレイト境界を使用します。

Fnトレイトは、標準ライブラリで用意されています。全てのクロージャは、以下のいずれかのトレイトを実装しています: Fn、FnMutまたは、FnOnceです。
「クロージャで環境をキャプチャする」節で、これらのトレイト間の差異を議論します; この例では、Fnトレイトを使えます。

Fnトレイト境界にいくつかの型を追加することで、このトレイト境界に合致するクロージャが持つべき引数と戻り値の型を示します。
今回のクロージャはu32型の引数を一つ取り、u32を返すので、指定するトレイト境界はFn(u32) -> u32になります。

リスト13-9は、クロージャとオプションの結果値を保持するCacher構造体の定義を示しています。

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T, //このジェネリックな型には、where T: Fn(u32) -> u32 のクロージャが入る。
    value: Option<u32>,
}

Cacher構造体は、ジェネリックな型Tのcalculationフィールドを持ちます。
Tのトレイト境界は、Fnトレイトを使うことでクロージャであると指定しています。calculationフィールドに保存したいクロージャは全て、
1つのu32引数(Fnの後の括弧内で指定されている)を取り、u32(->の後に指定されている)を返さなければなりません。

valueフィールドの型は、Option<u32>です。クロージャを実行する前に、valueはNoneになるでしょう。
Cacherを使用するコードがクロージャの結果を求めてきたら、その時点でCacherはクロージャを実行し、
その結果をvalueフィールドのSome列挙子に保存します。それから、コードが再度クロージャの結果を求めたら、
クロージャを再実行するのではなく、CacherはSome列挙子に保持された結果を返すでしょう。

たった今解説したvalueフィールド周りのロジックは、リスト13-10で定義されています。

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

呼び出し元のコードにこれらのフィールドの値を直接変えてもらうのではなく、Cacherに構造体のフィールドの値を管理してほしいので、
これらのフィールドは非公開になっています。

Cacher::new関数はジェネリックな引数のTを取り、Cacher構造体と同じトレイト境界を持つよう定義しました。
それからcalculationフィールドに指定されたクロージャと、vau128クロージャを直接呼ぶ代わりに、valueメソッドを呼びます。
このメソッドは、結果の値がself.valueのSomeに既にあるかどうか確認します; そうなら、クロージャを再度実行することなくSome内の値を返します。

self.valueがNoneなら、コードはself.calculationに保存されたクロージャを呼び出し、
結果を将来使えるようにself.valueに保存し、その値を返しもします。

リスト13-11は、リスト13-6の関数generate_workoutでこのCacher構造体を使用する方法を示しています。

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

クロージャを変数に直接保存する代わりに、クロージャを保持するCacherの新規インスタンスを保存しています。
そして、結果が必要な場所それぞれで、そのCacherインスタンスに対してvalueメソッドを呼び出しています。
必要なだけvalueメソッドを呼び出したり、全く呼び出さないこともでき、重い計算は最大でも1回しか走りません。

リスト13-2のmain関数とともにこのプログラムを走らせてみてください。
simulated_user_specified_valueとsimulated_random_number変数の値を変えて、いろんなifやelseブロックの場合全てで、
calculating slowlyは1回だけ、必要な時にのみ出現することを実証してください。
必要以上に重い計算を呼び出さないことを保証するのに必要なロジックの面倒をCacherは見るので、
generate_workoutはビジネスロジックに集中できるのです。



*Cacher実装の限界

値をキャッシュすることは、コードの他の部分でも異なるクロージャで行いたくなる可能性のある一般的に有用な振る舞いです。
しかし、現在のCacherの実装には、他の文脈で再利用することを困難にしてしまう問題が2つあります。

1番目の問題は、Cacherインスタンスが、常にvalueメソッドの引数argに対して同じ値になると想定していることです。
言い換えると、Cacherのこのテストは、失敗するでしょう:

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

このテストは、渡された値を返すクロージャを伴うCacherインスタンスを新しく生成しています。このCacherインスタンスに対して1というarg値で呼び出し、
それから2というarg値で呼び出し、2というarg値のvalue呼び出しは2を返すべきと期待しています。

このテストをリスト13-9とリスト13-10のCacher実装で動かすと、assert_eqからこんなメッセージが出て、テストは失敗します:

thread 'call_with_different_values' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs'

問題は、初めてc.valueを1で呼び出した時に、Cacherインスタンスはself.valueにSome(1)を保存したことです。
その後valueメソッドに何を渡しても、常に1を返すわけです。

単独の値ではなく、ハッシュマップを保持するようにCacherを改変してみてください。ハッシュマップのキーは、渡されるarg値になり、
ハッシュマップの値は、そのキーでクロージャを呼び出した結果になるでしょう。self.valueが直接SomeかNone値であることを調べる代わりに、
value関数はハッシュマップのargを調べ、存在するならその値を返します。
存在しないなら、Cacherはクロージャを呼び出し、arg値に紐づけてハッシュマップに結果の値を保存します。
 
現在のCacher実装の2番目の問題は、引数の型にu32を一つ取り、u32を返すクロージャしか受け付けないことです。
例えば、文字列スライスを取り、usizeを返すクロージャの結果をキャッシュしたくなるかもしれません。
この問題を修正するには、Cacher機能の柔軟性を向上させるためによりジェネリックな引数を導入してみてください。



*クロージャで環境をキャプチャする

トレーニング生成の例においては、クロージャをインラインの匿名関数として使っただけでした。
しかし、クロージャには、関数にはない追加の能力があります: 環境をキャプチャし、自分が定義されたスコープの変数にアクセスできるのです。
 
リスト13-12は、equal_to_x変数に保持されたクロージャを囲む環境からx変数を使用するクロージャの例です。

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

ここで、xはequal_to_xの引数でもないのに、
equal_to_xが定義されているのと同じスコープで定義されているx変数をequal_to_xクロージャは使用できています。

同じことを関数では行うことができません; 以下の例で試したら、コードはコンパイルできません:

fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}

エラーが出ます:

error[E0434]: can't capture dynamic environment in a fn item; use the || { ...
} closure form instead
(エラー: fn要素では動的な環境をキャプチャできません; 代わりに|| { ... }のクロージャ形式を
使用してください)
 --> src/main.rs
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^

コンパイラは、この形式はクロージャでのみ動作することさえも思い出させてくれています！

クロージャが環境から値をキャプチャすると、メモリを使用してクロージャ本体で使用できるようにその値を保存します。
このメモリ使用は、環境をキャプチャしないコードを実行するようなもっと一般的な場合には払いたくないオーバーヘッドです。
関数は、絶対に環境をキャプチャすることが許可されていないので、関数を定義して使えば、このオーバーヘッドを招くことは絶対にありません。

クロージャは、3つの方法で環境から値をキャプチャでき、この方法は関数が引数を取れる3つの方法に直に対応します:
所有権を奪う、可変で借用する、不変で借用するです。これらは、以下のように3つのFnトレイトでコード化されています:

    ・FnOnceは、クロージャの環境として知られている内包されたスコープからキャプチャした変数を消費します。
      キャプチャした変数を消費するために、定義された際にクロージャはこれらの変数の所有権を奪い、自身にムーブするのです。
      名前のうち、Onceの部分は、このクロージャは同じ変数の所有権を2回以上奪うことができないという事実を表しているので、
      1回しか呼ぶことができないのです。
    ・FnMutは、可変で値を借用するので、環境を変更することができます。
    ・Fnは、環境から値を不変で借用します。

クロージャを生成する時、クロージャが環境を使用する方法に基づいて、コンパイラはどのトレイトを使用するか推論します。
少なくとも1回は呼び出されるので、全てのクロージャはFnOnceを実装しています。キャプチャした変数をムーブしないクロージャは、FnMutも実装し、
キャプチャした変数に可変でアクセスする必要のないクロージャは、Fnも実装しています。
リスト13-12では、equal_to_xクロージャはxを不変で借用しています(ゆえにequal_to_xはFnトレイトです)。クロージャの本体は、xを読む必要しかないからです。

環境でクロージャが使用している値の所有権を奪うことをクロージャに強制したいなら、引数リストの前にmoveキーワードを使用できます。
このテクニックは、新しいスレッドにデータが所有されるように、クロージャを新しいスレッドに渡して、データをムーブする際に大概は有用です。

並行性について語る第16章で、moveクロージャの例はもっと多く出てきます。
とりあえず、こちらがmoveキーワードがクロージャ定義に追加され、整数の代わりにベクタを使用するリスト13-12からのコードです。
整数はムーブではなく、コピーされてしまいますからね; このコードはまだコンパイルできないことに注意してください。

fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // ここでは、xを使用できません: {:?}
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

以下のようなエラーを受けます:

error[E0382]: use of moved value: `x`
(エラー: ムーブされた値の使用: `x`)
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
                                  (値はここで(クロージャに)ムーブされた)
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
                                             (ムーブ後、値はここで使用された)
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
  implement the `Copy` trait
  (注釈: `x`が`std::vec::Vec<i32>`という`Copy`トレイトを実装しない型のため、ムーブが起きました)

クロージャが定義された際に、クロージャにxの値はムーブされています。moveキーワードを追加したからです。
そして、クロージャはxの所有権を持ち、mainがprintln!でxを使うことはもう叶わないのです。println!を取り除けば、この例は修正されます。

Fnトレイトのどれかを指定するほとんどの場合、Fnから始めると、コンパイラがクロージャ本体内で起こっていることにより、
FnMutやFnOnceが必要な場合、教えてくれるでしょう。

環境をキャプチャできるクロージャが関数の引数として有用な場面を説明するために、次のトピックに移りましょう:イテレータです。
