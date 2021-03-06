* トレイト: 共通の振る舞いを定義する

トレイトは、Rustコンパイラに、特定の型に存在し、他の型と共有できる機能について知らせます。 トレイトを使用すると、共通の振る舞いを抽象的に定義できます。
トレイト境界を使用すると、 あるジェネリックが、特定の振る舞いをもつあらゆる型になり得ることを指定できます。



* トレイトを定義する

型の振る舞いは、その型に対して呼び出せるメソッドから構成されます。異なる型は、それらの型全てに対して同じメソッドを呼び出せるなら、同じ振る舞いを共有することになります。
トレイト定義は、メソッドシグニチャをあるグループにまとめ、なんらかの目的を達成するのに必要な一連の振る舞いを定義する手段です。

例えば、いろんな種類や量のテキストを保持する複数の構造体があるとしましょう: 特定の場所から送られる新しいニュースを保持するNewsArticleと、
新規ツイートか、リツイートか、はたまた他のツイートへのリプライなのかを示すメタデータを伴う最大で280文字までのTweetです。

NewsArticle または Tweet インスタンスに保存されているデータのサマリーを表示できるメディア アグリゲータ ライブラリを作成します。
これをするには、各型のサマリーが必要で、インスタンスで summarize メソッドを呼び出してサマリーを要求する必要があります。
リスト10-12は、この振る舞いを表現するSummaryトレイトの定義を表示しています。

pub trait Summary {
    fn summarize(&self) -> String;
}

ここでは、traitキーワード、それからトレイト名を使用してトレイトを定義していて、その名前は今回の場合、Summaryです。
波括弧の中にこのトレイトを実装する型の振る舞いを記述するメソッドシグニチャを定義し、今回の場合は、fn summarize(&self) -> Stringです。

メソッドシグニチャの後に、波括弧内に実装を提供する代わりに、セミコロンを使用しています。
このトレイトを実装する型はそれぞれ、メソッドの本体に独自の振る舞いを提供しなければなりません。
コンパイラにより、Summaryトレイトを保持するあらゆる型に、このシグニチャと全く同じメソッドsummarizeが定義されていることが 強制されます。

トレイトには、本体に複数のメソッドを含むことができます: メソッドシグニチャは行ごとに並べられ、 各行はセミコロンで終わります。



*トレイトを型に実装する

今や Summary トレイトを使用して目的の動作を定義できたので、メディア アグリゲータでこれを型に実装できます。
リスト10-13は、 Summary トレイトを NewsArticle 構造体上に実装したもので、ヘッドライン、著者、そして地域情報を使ってsummarize の戻り値を作っています。
Tweet 構造体に関しては、ツイートの内容が既に280文字に制限されていると仮定して、ユーザー名の後にツイートのテキスト全体が続くものとして summarize を定義します。

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

型にトレイトを実装することは、普通のメソッドを実装することに似ています。違いは、implの後に、
実装したいトレイトの名前を置き、それからforキーワード、さらにトレイトの実装対象の型の名前を指定することです。
implブロック内に、トレイト定義で定義したメソッドシグニチャを置きます。各シグニチャの後にセミコロンを追記するのではなく、
波括弧を使用し、メソッド本体に特定の型のトレイトのメソッドに欲しい特定の振る舞いを入れます。

トレイトを実装後、普通のメソッド同様にNewsArticleやTweetのインスタンスに対してこのメソッドを呼び出せます。 こんな感じで:

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        // もちろん、ご存知かもしれませんがね、みなさん
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());

リスト10-13でSummaryトレイトとNewArticle、Tweet型を同じlib.rsに定義したので、 全部同じスコープにあることに注目してください。
このlib.rsをaggregatorと呼ばれるクレート専用にして、誰か他の人が私たちのクレートの機能を活用して自分のライブラリのスコープに定義された構造体にSummaryトレイトを実装したいとしましょう。
まず、トレイトをスコープに取り込む必要があるでしょう。use aggregator::Summary;と指定してそれを行えば、 これにより、自分の型にSummaryを実装することが可能になるでしょう。
Summaryトレイトは、 他のクレートが実装するためには、公開トレイトである必要があり、ここでは、リスト10-12のtraitの前に、 pubキーワードを置いたのでそうなっています。

トレイト実装で注意すべき制限の1つは、トレイトか対象の型が自分のクレートに固有(local)である時のみ、型に対してトレイトを実装できるということです。
例えば、Displayのような標準ライブラリのトレイトをaggregatorクレートの機能の一部として、 Tweetのような独自の型に実装できます。型Tweetがaggregatorクレートに固有だからです。
また、SummaryをaggregatorクレートでVec<T>に対して実装することもできます。 トレイトSummaryは、aggregatorクレートに固有だからです。

しかし、外部のトレイトを外部の型に対して実装することはできません。例として、 aggregatorクレート内でVec<T>に対してDisplayトレイトを実装することはできません。
DisplayとVec<T>は標準ライブラリで定義され、aggregatorクレートに固有ではないからです。
この制限は、コヒーレンス(coherence)、特に孤児のルール(orphan rule)と呼ばれるプログラムの特性の一部で、
親の型が存在しないためにそう命名されました。この規則により、他の人のコードが自分のコードを壊したり、その逆が起きないことを保証してくれます。
この規則がなければ、2つのクレートが同じ型に対して同じトレイトを実装できてしまい、 コンパイラはどちらの実装を使うべきかわからなくなってしまうでしょう。



*デフォルト実装

時として、全ての型の全メソッドに対して実装を要求するのではなく、トレイトの全てあるいは一部のメソッドに対してデフォルトの振る舞いがあると有用です。
そうすれば、特定の型にトレイトを実装する際、各メソッドのデフォルト実装を保持するかオーバーライドするか選べるわけです。

リスト10-14は、リスト10-12のように、メソッドシグニチャだけを定義するのではなく、 Summaryトレイトのsummarizeメソッドにデフォルトの文字列を指定する方法を示しています。

pub trait Summary {
    fn summarize(&self) -> String {
        // "（もっと読む）"
        String::from("(Read more...)")
    }
}
