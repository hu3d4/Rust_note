* 所有権 * ownership

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
