fn main() {
    let list_of_numbers = vec![1, 2, 3];
    /*
    イテレートされたlist_of_numbersを、map()で一つずつString型に変換して、
    最後に、collect()で配列(Vec<String>)に戻す。
    */
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    // ["1", "2", "3"]
    println!("{:?}", list_of_strings);
}
