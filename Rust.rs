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
