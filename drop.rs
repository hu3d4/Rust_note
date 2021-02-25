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
