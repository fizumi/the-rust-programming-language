# [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja)

The Book 個人メモ

## rustc

### version 確認

```bash
rustc --version
```

### コンパイル

Rust コンパイラ で `main.rs` をコンパイル

```bash
rustc main.rs
```

## cargo

### バージョンを確認する

```bash
cargo --version
```

### プロジェクトを作成する

```bash
cargo new hello_cargo --bin
```

### ビルド

```bash
cargo build
```

### 実行

```bash
./target/debug/hello_cargo
```

※ Cargo はビルドの結果を target/debug ディレクトリに格納する。（「リリースビルド」（cargo build --release / 実行は速いがコンパイル速度は遅い）の場合を除く）

### ビルド & 実行

```bash
cargo run
```

### コンパイルが通る確認のみ

```bash
cargo check
```

### クレートを更新して新バージョンを取得する

Cargo.lock ファイルに書かれているバージョンが変更される

```bash
cargo update
```

<!-- https://github.com/DavidAnson/markdownlint/tree/v0.23.1#configuration -->
<!-- markdownlint-disable MD024 -->

## 変数

- `let` で不変の変数を宣言
  - `let x = 5;`
- `let mnt` で可変の変数を宣言
  - `let mut x = 5;`
- シャドーイング
  - 意義：前に定義した変数と同じ名前の変数を新しく宣言する
  - 効果：変数の型を変更できる
  - 例：`let mut x = x;`
  - shadow の主語と目的語：
    - 新しい変数（`=`の左側）は覆い隠す（shadow）する
    - 古い変数（`=`の右側）は覆い隠される（be shadowed）
- 初期値なしの宣言も可能
  - 例：`let r;`
  - ただし、値を与える前に変数を使用しようとすれば、コンパイルエラーになり、 これは、確かに Rust では null 値は許可されないことを示す

## 定数

- `const` キーワードで宣言
  - 注釈（`:` + 型）が必須
  - `mut` キーワードは使えない
- グローバルスコープに定義可能
- 関数呼び出し結果や、実行時に評価される値はセットできない

## データ型

- 値は全て、何らかのデータ型に属する
- データ型はスカラー型と複合型に分けられる

### スカラー型

- 数値型
  - 整数型
    - 小数部分のない数値
    - [Rust の整数型はリンク先を参照](https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html#%E6%95%B4%E6%95%B0%E5%9E%8B)
    - 整数リテラル
      - 数値以外で使える文字
        - 型接尾辞
          - 例：`57u8`
        - 見た目の区切り記号(visual separator)
          - 例：`1_000`
    - 整数型の基準は`i32`型
      - 64 ビットシステム上でも、通常はこの型が最速
  - 浮動小数点数型
    - 数値に小数点がついたもの
    - IEEE-754 規格に従って表現される
    - Rust の浮動小数点型は、`f32` と `f64`
    - 基準型は `f64`
      - ∵ `f32` とほぼ同スピードにもかかわらず、より精度が高い
- 論理値型
  - `bool` と指定
  - `true` と `false`
  - `!`演算子で反転可能
- 文字型（char 型）
  - シングルクォートを使う
  - ユニコードのスカラー値を表す
    - U+0000 から U+D7FF までと U+E000 から U+10FFFF までの範囲

### 複合型

- 意義：複数の値を一つの型にまとめた型
- 複数の型が推論される可能性がある場合、型注釈をつけることが必須。（TODO: 要補足）
- タプル型
  - 例: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
  - 分配（destructuring）
    - `let tup = (500, 6.4, 1);`
    - `let (x, y, z) = tup;`
  - 要素へのアクセスはピリオド`.`
    - `let tup = (500, 6.4, 1);`
    - `let five_hundred = x.0;`
    - `let one = x.2;`
- 配列型
  - 例: `let a = [1, 2, 3, 4, 5];`
  - 要素へのアクセス JavaScript と同じ
    - `let a = [1, 2, 3, 4, 5];`
    - `let first = a[0];`
    - `let second = a[1];`
    - 無効なアクセス
      - **実行時**エラーとなる
      - 例:
        - `let a = [1, 2, 3, 4, 5];`
        - `let second = a[10];`
  - 配列の全要素は、 同じ型でなければならない。
  - 固定長（動的な要素数の変更は不可）
  - 利用場面
    - ヒープでなくスタックを使う場合
    - 要素数が固定のデータを使う場合
      - 例: 年月日の月
  - JavaScript の配列は Rust のベクタ型に近い

## プログラムの状態

- panicked
  - プログラムがエラーで終了したことを表す Rust 用語

## 文と式

- 文（Statements）
  - なんらかの動作をして値を返さない命令
  - 例
    - 変数宣言（let キーワード）
      - NG 例: `let x = (let y = 6);`
    - 関数定義（fn キーワード）
- 式（Expressions）
  - 評価され値になる
  - 終端にセミコロンを含まない
  - 例
    - 関数呼び出し
    - マクロ呼び出し
    - ブロック（`{}`）
    - if 式

## コメント

JavaScript と同じ

## フロー制御

- 条件式
  - bool 型でなければならない（論理値以外の値が、自動的に論理値に変換されることはない）
- if 式
  - `if` + 条件式 + `{}`
  - `if` + 条件式 + `{}` + `else` + `{}`
  - `if` + 条件式 + `{}` + ...[`else if`+ 条件式 + `{}`] + `else` + `{}`
- 繰り返し: loop, while, for
  - `loop` + `{}`
    - ブロック内を無限ループさせる
    - `break` で抜ける
  - `while` + 条件式 + `{}`
  - `for` + 変数 + `in` + イテレータ + `{}`

## 演算子

- 数値演算
  - 数値型に適用される
  - JavaScript と同じ
    - `+、–、*、/、%`
- `!`を使用すると、普通の関数ではなくマクロを呼ぶ
  - 例：`println!("Hello, world!");`
- range expression
  - `start..end`
    - start ≦ x ＜ end
  - `1..101` と `1..=100` は同じ（1 から 100 まで）

## コンパイル時におけるメモリ安全性の保証

### メモリ

- メモリ管理：ガベージコレクタ(GC)、allocate と free、所有権システムの３つ
  1. ガベージコレクタ(GC)
     - メモリ確保・解放を明示的に行う必要がなく便利
     - 動作が遅くなる
     - memo：メモリの制御は不可能（完全に自動）
  2. プログラマによる allocate と free（プログラマが明示的にメモリを確保・解放する）
     - メモリの確保・解放は歴史的にも難しいプログラミング問題の一つ
       - allocate と free は完全に 1 対 1 に対応していないといけない
         - free が重複すると**二重解放エラー**（メモリ安全性上のバグの一つ）となる
     - memo：メモリの制御は手動
  3. 所有権システム
     - Rust 独自
     - メモリを所有している変数がスコープを抜けたら、 メモリは自動的に解放（これを「drop / ドロップ」という）される
     - 慣れれば安全かつ効率的なコードを構築できる
     - memo：特定のルールに従うことでメモリの制御が可能。
- スタックとヒープ
  - 実行時にコードが使用できるメモリの一部
- スタック
  - データアクセスが高速
    - 理由 1：last in, first out だから
    - 理由 2：スタック上のデータは全て既知の固定サイズだから
  - 配置されるデータ
    - 関数のローカル変数（ポインタ + α）等
    - コンパイル時に存在が確定している固定長データ（リテラル）
- ヒープ
  - データアクセスが低速
    - 理由：データが飛び飛びの場所にある（隔離されている）場合があるから
  - 配置されるデータ
    - コンパイル時にサイズがわからなかったり、サイズが可変のデータ
  - allocate（on the heap）の構成
    - データのスペースの確保
    - スペースへのポインタの取得
- 関数とメモリ
  - 関数開始時（呼び出し時）
    - 関数に渡された値（or ヒープのデータへのポインタ）と、 関数のローカル変数がスタックに配置される
  - 関数終了時
    - スタックに配置されたデータはスタックから取り除かれる。
- スコープ
  - 変数は宣言された地点から、宣言されたブロックの終わりまで有効
- 文字列
  - 文字列リテラル
    - 静的、不変（中身はコンパイル時に判明しているので、テキストは最終的なバイナリファイルに直接ハードコードされる）
      - ⇒ 高速、効率的、`'static`ライフタイム
    - 文字列スライスの一種
  - String 型
    - 動的、可変
      - ヒープにメモリを確保される

### 所有権

- 所有権システム
  - 機能
    - どの部分のコードがどのヒープ上のデータを使用しているか把握する
    - ヒープ上の重複するデータを最小化する
    - メモリ不足にならないようにヒープ上の未使用のデータを掃除する
  - ルール
    - Rust の値は、**所有者**（owner）と呼ばれる変数と対応している
    - いかなる時も所有者は一つである
    - 所有者がスコープから外れたら、値は破棄される
- ムーブ（Move）
  - memo：ムーブでは、スタックメモリのみが変化する
  - 例:
    - `let s1 = String::from("hello");`
      - スタック
        - s1: { ptr: ピープへのポインタ, len: 5, capacity: 5 }
      - ヒープ
        - (index, char) × len
    - `let s1 = s1;`
      - スタック
        - s1: { ptr: ピープへのポインタ, len: 5, capacity: 5 } （無効化される）
        - s2: { ptr: ピープへのポインタ, len: 5, capacity: 5 } （コピーされる）
      - ヒープ
        - (index, char) × len （ヒープのデータはコピーされない）
  - Rust では、 自動的にデータの _deep copy_ が行われることは絶対にない
    - ⇒: 実行時性能が良い
    - ⇒: _deep copy_ をしたい場合、`clone`メソッドを使う
- コピー（Copy）
  - 例:
    - `let x = 5;`
    - `let y = x;`
    - `println!("x = {}, y = {}", x, y);` （← 有効）
  - 静的に確定している値はムーブではくコピーされる
    - memo：静的に確定している値は allocate が不要なので、「コピー元変数無効化による二重解放エラー」に対する心配は無用
  - ムーブではなくコピーとなる型
    - あらゆる数値型
    - 論理値型, 文字型
    - コピーとなる型から構成されるタプル
- 所有権と関数
  - 別の変数に値が移る場面ではムーブ（or コピー）が生じる
    - 具体例 1：関数に変数を渡すことでムーブ（or コピー）が生じる
    - 具体例 2：関数の戻り値として評価される変数にもムーブ（or コピー）が生じる

### 参照と借用

- 参照を受け取るだけでは、ムーブは発生しない
- 借用（borrowing）
  - 意義：関数の引数に参照を取ること
    - 例: `fn calculate_length(s: &String) -> usize { s.len() }`
      - memo：上記例では「自動参照および参照外し（automatic referencing and dereferencing）」が使われている
  - 借用された実引数（参照）は不変（参照先を変更することはできない）
  - 用途：呼出し元（main 関数等）から所有権を奪わないようにするためのテクニック
- 不変な参照（immutable references）
  - 特定のスコープに内に、いくつでも作ることができる。
- 可変な参照（Mutable References）
  - 例：（TODO: コード領域を作って移動）
    - change 関数
      - `fn change(some_string: &mut String)` 可変変数の参照の受取り
      - `{ some_string.push_str(", world"); }` 変更を加える
    - main 関数
      - `let mut s = String::from("hello");` 可変変数の定義
      - `change(&mut s);` 可変な参照として渡す
  - 制約 1：特定のスコープにおいて、一つのデータに対して同時に二つ以上の可変な参照を作ることはできない。
    - 例：
      - `let r1 = &mut s;`
      - `let r2 = &mut s;` （エラー）
    - 制約の利点：データ競合をコンパイル時に検知・防止できる
      - データ競合
        - 意義：未定義の振る舞いを引き起こし、実行時に追いかけようとした時に特定し解決するのが難しい問題
        - 発生条件：
          - 2 つ以上のポインタが同じデータに同時にアクセスする。
          - 少なくとも一つのポインタがデータに書き込みを行っている。
          - データへのアクセスを同期する機構が使用されていない。
  - 制約 2：同一スコープ上では、可変と不変な参照を同時に存在させることはできない。
    - 例：
      - `let r1 = &s;`
      - `let r2 = &s;` （不変な参照は２つ以上作ることができる）
      - `let r3 = &mut s;` （エラー）
    - memo：不変参照の使用者は、それ以降に値が変わらないことを想定していており、値に変更を加えられる状況を作ることはその使用者に不意打ちとなる可能性がある
- Dangling References
  - ダングリングポインタ
    - 意義：他人に渡されてしまった可能性のあるメモリを指すポインタ
    - 発生原因：その箇所へのポインタを保持している間に、 メモリを解放してしまう
    - Rust では発生しない
      - Rust では参照は常に有効でなければならなず、ダングリングポインタの発生はコンパイルエラーとなる

### スライス型

- 意義：所有権のない別のデータ型
- 機能：コレクション全体ではなく、その一部を参照する
- 文字列スライス
  - 生成方法：
    - コレクション + `[starting_index..ending_index]`
      - starting_index はスライスの最初の位置
      - ending_index はスライスの終端位置よりも、 1 大きい値
    - スライスの最初の要素が対象コレクションの最初の要素である場合の糖衣構文
      - `let slice = &s[0..2];`
      - `let slice = &s[..2];`
    - スライス最後の要素が対象コレクションの最後の要素である場合の糖衣構文
      - `let len = s.len(); let slice = &s[3..len];`
      - `let slice = &s[3..];`
    - 全て取得する場合
      - `&s[..]`
  - 内部的データ構造：
    - 開始地点
    - スライスの長さ：ending_index から starting_index を引いたもの
  - 文字列リテラルは文字列スライス
  - 型：`&str`
- 他のスライス
  - 例：
    - `let a = [1, 2, 3, 4, 5]; let slice = &a[1..3];`
      - 型：`&[i32]`

## struct（構造体 / structure）

- 意義：
  - 複数の関連した値を意味のあるグループにまとめる
  - 各フィールドに名前を付けられる
  - 独自の**データ型**として扱われる
- 補足：インスタンス全体が可変でなければフィールドを変更することはできない
- 役割：プログラムの意図を明示的にする
  - 例：関数の引数に使う
    1. `fn area(width: u32, height: u32) -> u32 { width * height }`
       - 評価：width, height という強い関連性を持つ情報があるが、その関連性を示す情報がない
    2. `fn area(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }`
       - 評価：相互に関係性があるデータを１つにまとめている点は良いが、タプルの順番が何を意味しているのか分からない
    3. `fn area(rectangle: &Rectangle) -> u32 { rectangle.width * rectangle.height }`
       - Rectangle：`struct Rectangle { width: u32, height: u32, }`
       - 評価：上記２つの悪い点を解消している
- 特殊な構造体
  - tuple structs: タプルを独自の型として扱う
  - unit-like structs：一切フィールドのない構造体

### コード

#### struct definition

```rust
struct User {
    username: String,
    active: bool,
}
```

#### creating an instance

```rust
let user1 = User {
    username: String::from("someusername123"),
    active: true,
};
```

#### field init shorthand

変数とフィールドが同じ名前の時に有効

```rust
fn build_user(username: String, active: bool) -> User {
    User {
        username: username, // field init shorthand を使わない場合
        active, // field init shorthand を使う場合
    }
}
```

#### struct update syntax

`..`という記法により、 明示的にセットされていないフィールドは、与えられたインスタンスのフィールドと同じ値になる。

```rust
let user2 = User {
    username: String::from("someusername123"),
    ..user1 // struct update syntax
};
```

#### tuple structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let white: Color = Point(255, 255, 255); // ❌ 違う型
```

#### `Debug` trait

❌ error

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

⭕

```rust
// add the `derive` annotation to derive引き出す the Debug trait
#[derive(Debug)] // `derive` annotation ✔
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // the specifier「:?」tells 「println!」
    // that we want to use an output format called Debug
    println!("rect1 is {:?}", rect1);  // specifier「:?」 ✔
    // output:
    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1); // specifier「:#?」 ✔
    /* output:
       rect1 is Rectangle {
           width: 30,
           height: 50
       } */
}
```

## メソッド （Method Syntax）

- 関数との違い
  - 特定の文脈で定義される
    - 文脈：
      - struct
      - enum
      - trait object
  - 第一引数には必ず`self`を使う
    - `&self`: 読込み専用（インスタンスのデータを読み込みたいだけ）
    - `&mut self`: 読込み＋書込み（メソッドを呼び出したインスタンスを変更したい）
    - `self`: 所有権奪取（self を何か別のものに変形し、変形後に呼び出し元が元のインスタンスを使用できないようにしたい場合など限られた場面でのみ使う）
- 関数の代替としてメソッドを使う主な利点
  - 特定の型を繰り返し書かずに済む（代わりに self キーワードを書く）
  - ある特定の型のインスタンスを使ってできることを一つの`impl`ブロックにまとめて体系化できる
- メソッドでは _automatic referencing and dereferencing_ （自動参照・自動参照外し機能）が発動する
  - 以下のようには書か**ない**
    - `pointer->something()`
    - `(*pointer).something()`
  - 以下は同じ
    - `pointer1.something(&pointer2);`
    - `(&pointer1).something(&pointer2);`
- _associated functions_ （関連関数）
  - 特定のオブジェクトではなく特定の型に対して実装された関数
  - impl ブロック内の self を引数に取らない関数
  - メソッドではない
  - 呼び出し方：構造体名 + `::` + 関連関数 （例：`String::from`）
  - 用途：
    - 名前空間を分ける
    - 構造体の新規インスタンスを返すコンストラクタ

### コード

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with More Parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

/* 以下のように impl ブロックを分けることも可能
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}*/

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

## Enum

- **取りうる値すべて**を列挙する
- enum の値は、その列挙子（_variants_）のいずれか一つになる
- 独自の**データ型**を形成する
- `impl`を使ってメソッドを定義できる
- コンパイラが、プログラマが処理すべき場面全てを処理しているかどうかをチェックできるようになる

### コード

❌ 冗長なコード

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

⭕ 簡潔なコード

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

Enum は 独自のデータ型を形成し、各列挙子は同じ型として扱われる

```rust
// 前のコードの続き

fn route(ip_addr: IpAddr) { /* --snip-- */ }
// home と loopback は同じ型（IpAddr）なので以下は OK
route(home)
route(loopback)
```

## match

- フロー制御演算子
- 全てのありうるパターンを網羅していない場合はコンパイルエラーとなる
- 文法：`match` + 式 + `{` + ...アーム `}`
  - アーム
    - 文法：パターン [ + マッチガード] + `=>` + 式 [ + `,`]
    - パターン
      - リテラル値、変数名、ワイルドカードやその他
      - ワイルドカード（`_`／プレースホルダ）：全てにマッチ
        - case 文の default ラベル みたいなもの
      - `ref` キーワード
        - 値にマッチし、それへの参照を返す
        - ムーブを防ぐ
      - `&`
        - 参照にマッチ
    - マッチガード：
      - アームのパターンに追加できる条件式
      - アームのコードが実行されるにはマッチガードが真でなければいけない

## if let

値が一つのパターンにマッチした時にコードを走らせ、 他は無視する match の糖衣構文と言える

### コード

❌ 冗長なコード

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

⭕ 簡潔なコード

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

（以下の例だと冗長だが）`else` も使える

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
} else {
    ()
}
```

## モジュールシステム

- パッケージ：クレートをビルドし、テストし、共有することができる Cargo の機能
- クレート：ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群
- モジュール と use：これを使うことで、パスの構成、スコープ、公開するか否かを決定できます
- パス：要素（例えば構造体や関数やモジュール）に名前をつける方法
- アイテム：関数、メソッド、構造体、列挙型、モジュール、および定数
  - memo：「item」を「要素」と訳すと element の訳と被るので、「アイテム」と記載

### クレート

- クレートルート（crate root）：Rust コンパイラの開始点となり、クレートのルートモジュールを作るソースファイル
  - 例：
    - src/main.rs
    - src/lib.rs
- 種類：バイナリかライブラリのどちらか
  - バイナリクレート
    - 実行可能形式
    - 単体で実行することを意味する
      - ⇒ 他のクレートから呼び出されない
  - ライブラリクレート
    - 他のプログラム（クレート）が呼び出して使用できる関数を公開する

### パッケージ

- 意義：ある機能群を提供する 1 つ以上のクレート
- 特徴：Cargo.toml ファイルを持つ
  - Cargo.toml：クレートをどのようにビルドするかを説明するファイル
- ルール
  - パッケージは少なくとも 1 つのクレートを持たなければならない
  - **ライブラリ**クレートを 2 個以上持つことはできない
  - **バイナリ**クレートはいくらでも持っていて良い

### モジュール

- 効果：クレート内のコードをグループ化し、可読性と再利用性を上げる
- 比喩：ファイルシステムのディレクトリ
- 機能：
  - ネストできる
    - ⇒: モジュールツリーを形成できる
  - アイテムの _privacy_ の制御
    - iow: *privacy boundary*の定義
    - ⇒: 関数や構造体といった要素を非公開にしたければ、モジュールに入れればよい
- 関連概念
  - _privacy_：
    - 意義：外部で使える (公開 _public_) のか、使えない (非公開 _private_) のかというアイテムの性質
    - デフォルト：_private_ （非公開）
      - memo: _private_ のメリット：非公開にしたコードは外部のコードを壊さないので、変更時に影響範囲を把握するのが楽
      - `pub` キーワードで公開（_public_）に変更可能
    - 親子間のプライバシー
      - 親モジュールのアイテムは子モジュールの非公開アイテムを参照できない
      - 子モジュールのアイテムはその祖先モジュールのアイテムを参照できる
        - ∵: 子モジュールは自分が定義された文脈を把握できる
    - 兄弟間（同じモジュール内で定義されているアイテム）のプライバシー
      - デフォルトで（非公開でも）互いに参照可能
    - 構造体：構造体のプライバシーと構造体のフィールドのプライバシーは別
    - Enum：Enum を公開すると列挙子も公開される
  - _privacy boundary_（プライバシー境界）：外部のコードが呼び出したり、依存したりしてはいけないかどうかのライン
- シンタックス：`mod` + モジュール名 + `{` + アイテム（モジュールを含む） + `}`

### パス

- 意義：モジュールツリー内のアイテムを取得する
- シンタックス：
  - スタートとなる識別子：
    - 絶対パス：
      - クレートの名前
      - `crate`
    - 相対パス：
      - `self`
      - `super`
        - ファイルシステムパスの`..`と同じ
        - `super::super::foo` と連続させることも可能
      - 今のモジュール内の識別子
  - 一つ以上の識別子をダブルコロン（`::`）で仕切る
- 補足：
  - 相対パスを使うか絶対パスを使うかは、プロジェクトによって決めましょう。
    - あるアイテムを定義するコードを移動させる場合、そのアイテムを使うコードと...
      - **別々に**移動させる可能性が高い => 絶対パス
      - **一緒に**移動させる可能性が高い => 相対パス
- `use` キーワード
  - 効果：パスをスコープに持ち込み、それ以降はパス内の要素がローカルにあるかのように呼び出すことができる
  - 文法：
    - `use` + パス [ + `as` + 名前] + `;`
      - 例：
        - `use std::collections::HashMap;`
        - `use std::io::Result as IoResult;`
    - nested paths： use 文の数を減らせる
      - 例 1
        - not nested:
          - `use std::cmp::Ordering;`
          - `use std::io;`
        - nested: `use std::{cmp::Ordering, io};`
      - 例 2
        - not nested:
          - `use std::io;`
          - `use std::io::Write;`
        - nested: `use std::io::{self, Write};`
    - ファイルの先頭で宣言する必要はない
  - 比喩：ファイルシステムにおいてシンボリックリンクを張る
  - 慣例：
    - 種類別：
      - 関数：関数の親モジュールを use する
      - 構造体・Enum：個別にそれ自体を use する
    - 補足：慣例はあくまでも慣例であり、はっきりとした理由はない
    - 名前の衝突の回避（以下のどちらでもよい）
      - 親モジュールを use する
      - 衝突する名前のどちらかに as を用いて名前を返る
  - 再公開（re-exporting）
    - `pub use` を用いる
  - memo:
    - [プレリュード](https://doc.rust-lang.org/std/prelude/index.html)（prelude ＝ Rust がすべての Rust プログラムに自動的にインポートするもののリスト）に含まれているものは use する必要はない
    - `std`（標準ライブラリクレート）は `crates.io` からダウンロードする必要はないが use する必要がある
  - glob 演算子 `*`
    - 効果：パスにおいて定義されているすべての公開要素をスコープに持ち込む
    - 用途：
      - テスト対象の use

## コレクション

- 複数の値を含むことができる
- コレクションが指すデータはヒープに確保される
  - 組み込みの配列とタプル型とは異なる
  - データ量はコンパイル時にわかる必要はなく、 プログラムの実行にあわせて、伸縮可能
- 各種のコレクションから状況に応じた最適なものを選び取る必要がある
- 種類
  - ベクタ型：可変長の値を並べて保持
  - 文字列：文字のコレクション
  - ハッシュマップ：
    - 値を特定のキーと紐付ける
    - より一般的なデータ構造であるマップの実装の一つ
  - [その他](https://doc.rust-lang.org/std/collections/index.html)

### ベクタ（`Vec<T>`）

- メモリ上に値を隣り合わせに並べる単独のデータ構造
- 同じ型の値しか保持できない
  - Enum と組み合わせると複数の型を保持できる
- 生成：`let v: Vec<i32> = Vec::new();`
  - ※ TypeScript と同様、値がない場合は当然ジェネリックは効かないので、型注釈が必要
- マクロによる生成：`let v = vec![1, 2, 3];`
  - ※TypeScript と同様、初期値が与えられれば型注釈は必要ない
- 更新：`push`メソッド
  - 例：`let mut v = Vec::new(); v.push(5);`
    - ※ TypeScript とは異なり、生成後でもコンパイルが型を推論してくれるので注釈は必要でなくなる（NOTE）
- 破棄：スコープを抜ければ自動的に破棄される（ドロップ）
- 取得：`let v = vec![1, 2, 3, 4, 5];`を前提とすると...
  - 添字記法（indexing syntax）：`let third: &i32 = &v[2];`
    - 存在しない要素を参照するとパニックが発生する
  - get メソッド：`let third: Option<&i32> = v.get(2);`

#### 走査

不変

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

可変

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

#### Enum を利用した複数の型の保持

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### 文字列

- 構成：
  - バイトのコレクション
  - それらのバイトをテキストとして解釈した時に有用なメソッド
- 核となる型：文字列スライス（`str`）
  - 通常借用された形態`&str`で目にすることが多い
  - Rust には、言語の核としては、この 1 種類しか文字列型が存在しない
    - `String`型は標準ライブラリで提供されるものであり、言語の核として存在しているものではない
    - 標準ライブラリで提供されるその他の文字列型：OsString、OsStr、CString、CStr
  - 別の場所に格納された UTF-8 エンコードされた文字列データへの参照
- Rust における「文字列」とは：通常、`String`と文字列スライスの`&str`のこと

#### `String`型

- 添え字アクセス
  - 不可（エラーとなる）
  - `String` は `Vec<u8>`のラッパ
    - 表現方法として、バイト値、文字、書記素クラスタ（人間が文字と呼ぶものに一番近い）など複数考えられる。
      - 例えば、ある文字列はバイト列として見ると 18 バイト、Unicode スカラー値としてみたら 6 つの`char`、書記素クラスタとして見たら、4 つの文字となったりする。添え字でアクセスする場合、どの表現を前提としてアクセスするのか不明確。
    - データとしてはただのバイト列なので、データの並びに index でアクセスするとバイト値を返すことになるが、これは文字を扱いたいユーザーの望む動作ではないことは明らか
      - かといって書記素クラスタ等で処理するにはデータの走査が必要になり、「添え字アクセスは`O(1)`である」という期待に反する動作になってしまう
- スライス
  - 文字の境界で適切にスライスしないとパニックを発生させる
- 走査
  - `chars`メソッド：`char`型のイテレータを返す
  - `bytes`メソッド：各バイトのイテレータを返す
- 補足
  - 書記素クラスタ
    - 文字列から得る方法は複雑なので、この機能は標準ライブラリでは提供されていない
    - 必要な場合、crates.io からクレートを探す

#### `+`演算子

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // ※
```

- 内部的に`add`メソッドを使用
  - シグニチャ：`fn add(self, s: &str) -> String`
    - ⇒ s1 はムーブされ、もう使用できない
    - `&str`に`&String`を渡せる理由：
      - ∵ add メソッド呼び出しの際、コンパイラは、参照外し型強制（_deref coercion_）を行う
        - 上記例では`&s2`を`&s2[..]`に型強制（_coerce_）される
    - `self`には`&`がついていない ⇒ `add`は`self`の所有権を奪う
      - ⇒ 実行効率が良い

#### format!マクロ

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

// let s = s1 + "-" + &s2 + "-" + &s3;
let s = format!("{}-{}-{}", s1, s2, s3);
```

- println!と同様の動作をしますが、 出力をスクリーンに行う代わりに、中身を String で返す
- format!を使用したコードは引数の所有権を奪わない

### ハッシュマップ

- ハッシュ関数
  - 役割：キーと値のメモリ配置方法を決める
  - デフォルト：サービス拒否(DoS)アタックに対して抵抗を示す暗号学的に安全なハッシュ関数を使用
    - ○：安全性が高い
    - ×：利用可能な最速のハッシュアルゴリズムではない
  - 異なる hasher を指定することで別の関数に切り替えることができる
    - hasher：BuildHasher トレイトを実装する型
- キーは全て同じ型でなければならず、 値も全て同じ型でなければなりません。
- 補足：
  - 型注釈でアンダースコアを使用：コンパイラに、ベクタのデータ型に基づいてハッシュマップが含む型を推論させる
- 取得
  - 例：`let team_name = String::from("Blue"); let score = scores.get(&team_name);`
  - 走査：
    - 例：`for (key, value) in &scores { println!("{}: {}", key, value); }`
- 更新
  - 上書き（upsert）：
    - 例：`let mut scores = HashMap::new(); scores.insert(String::from("Blue"), 10);`
  - キーに値がなかった時のみ値を挿入する：
    - 例：`scores.entry(String::from("Yellow")).or_insert(50);`

#### 古い値に基づいて値を更新する

TODO: 実装して動かしてみる

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

## エラー処理

- Rust では、エラーは大きく二つに分類されます: 回復可能なエラーと回復不能なエラー
  - 回復可能なエラー：`Result<T, E>`値で処理
  - 回復不能なエラー：`panic!`マクロで処理
  - Rust には例外が存在しない
    - プログラムを終了させる手段（`panic!`）はあるが、例外をキャッチするといった機能は無い

### 回復不能なエラー

- `panic!`マクロ
  - 機能：
    - 失敗メッセージの表示
    - マクロを使用したソースコードの位置の表示
    - スタックを巻き戻し掃除して、終了
- バックトレース：
  - panic マクロ呼出し元に至るまでに呼び出された全関数の一覧
  - `RUST_BACKTRACE`環境変数でトレースする数を調整可能
  - OS や Rust のバージョンによって、出力の詳細は変わる可能性がある
- デバッグシンボル：
  - バックトレースを得るかどうかの設定
  - `--release`オプションなしで `cargo build` や `cargo run` を使用していれば、標準で有効

```rust
fn main() {
    panic!("crash and burn");
    /*
      thread 'main' panicked at 'crash and burn', src/main.rs:2:4
      note: Run with `RUST_BACKTRACE=1` for a backtrace.
    */
}
```

#### パニックに対する対処法

「巻き戻し」と「異常終了」とが存在する。

- Cargo.toml ファイルによって選択が可能
  - [profile]欄：
    - `panic = 'abort'`：異常終了
- 巻き戻し：
  - 言語がスタックを遡り、 遭遇した各関数のデータを片付ける
  - すべきことが多い
    - ⇒ 実行可能ファイルを大きくする
- 異常終了：
  - 片付けをせずにプログラムを終了させる
  - プログラムが使用していたメモリは、 OS が片付ける
    - ⇒ 実行可能ファイルを小さくする

リリースモード時に異常終了するようにしたければ、Cargo.toml ファイルに以下を追記する。

```rust
[profile.release]
panic = 'abort'
```

### 回復可能なエラー

Result Enum を使用する

- match を用いて自分で制御する
- Result 型のメソッドを利用する
  - unwrap
    - Result 値が Ok 列挙子なら、Ok の中身を返す
    - Result が Err 列挙子なら、 panic!マクロを呼ぶ
  - expect
    - panic!のエラーメッセージを渡せる unwrap
- エラーの伝播（_Propagating Errors_）：関数の戻り値を Result 型にして、結果を制御する役割（or 権限）を呼出し元に移譲する
  - `?`演算子：Propagating Errors のショートカット
    - 文法： `Result` + `?` + `;`
      - Result が Ok ならその中身として評価され、プログラのを継続する
      - Result が Err なら、「`return` + from で変換された Err」のような動作になる
        - `?`演算子が`from`関数を呼び出す
          - エラー型が現在の関数の戻り値型で定義されているエラー型に変換される
        - `?`演算子が変換されたエラー型を`return`する
    - 制約：`?`演算子は、Result を返す関数でしか使用できない
      - main 関数は、戻り値が()なので main 関数内で`?`演算子は使えない

#### コード

`?`演算子なしの Propagating Errors

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

`?`演算子ありの Propagating Errors

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

`?`演算子ありの Propagating Errors （+ メソッドチェーン（Chaining method））

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

### panic! or Result

- Result が第一候補（出来るだけ Result を使うべき）
  - Result を使えば、呼び出す側は、panic! を使うか、プログラムの実行を継続するかを選択することができる
- `panic!`を使う場合
  - リリース前の特定の場合
    - プロトタイプの場合、あまり堅牢に作らなくてもよいから panic!で済ませてしまうのも一つの手段
      - 実際に開発段階に入った段階で、panic! を使う部分を修正していく
    - テストコードの場合は、panic! ですぐに失敗を知らせるようにするのも手段として有効
  - コードが悪い状態に陥る可能性があるとき
    - iow: 呼出し側のプログラマにコードを書き直させる必要がある場合
    - ※ 悪い状態（_bad state_）：
      - 何らかの前提、保証、約束、不変性が破られた状態（例えば、無効な値、 矛盾する値、欠損値がコードに渡された状態）
      - または以下を１つ以上満たす場合
        - 悪い状態に陥ることが予期できる場合？（TODO ここは理解ができてない／悪い状態 is「偶然起こることが予想される事象」でない）
        - この時点以降、この悪い状態にないことを頼りにコードが書かれているとき。
        - 使用している型にこの情報をコード化するいい手段がないとき。
    - 例：
      - ライブラリの開発者が利用者に対して、「利用方法間違ってるよ」と知らせたい場合
      - セキュリティーの観点から実行すべきでないことを呼出し元に伝える場合（境界外へのメモリアクセス（データ構造に属しないメモリにアクセスを試みられた）など）
- `unwrap`を使う場合
  - ある Result について、Ok になることが、（静的に）確定的な場合
    - 例えば、コンパイラはある文字列リテラルの内容・意味を理解できない。このように、プログラマはコンパイラよりも多くの情報を持っている。そのため、コンパイラにとっては Result は分からないが、プログラマにとっては確定的に Ok と判断できる場合もある。

## ジェネリクス

- 概念の重複を効率的に扱う道具
- 具体型や他のプロパティの抽象的な代役
- 関数、構造体、enum、メソッドなどのアイテムを定義する際に使用できる
- コードで多くのジェネリックな型が必要な時は、 コードの小分けが必要なサインかもしれない
- rust では、コードの単相化をコンパイル時に行うことで、具体的な型があるコードよりもジェネリックな型を使用したコードの方が遅くならないようにしてある
  - 単相化（monomorphization）は、コンパイル時に使用されている具体的な型を入れることで、 ジェネリックなコードを特定のコードに変換する過程のこと
    - iow: コンパイラは、ジェネリックなコードが呼び出されている箇所全部を見て、 ジェネリックなコードが呼び出されている具体的な型のコードを生成する
    - iow: ジェネリックなコードを実行すると、 それぞれの定義を手作業で複製した時のように振る舞う

### コード

#### 構造体とメソッド 1

ジェネリックな型引数 T に対して特定の具体的な型がある構造体にのみ適用される impl ブロック

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Point<f32>にはdistance_from_originというメソッドが存在するが、 Tがf32ではないPoint<T>の他のインスタンスにはこのメソッドが定義されない
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

#### 構造体とメソッド 2

一部のジェネリックな引数は impl で宣言され、他の一部はメソッド定義で宣言される場面

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

## トレイト

- 大要：共通の振る舞いを定義する
- 機能：ジェネリックな型にトレイトを組み合わせることで、ジェネリックな型を、「特定の振る舞いのある型」に制限できます。
- 類似：TypeScript において interface を class で implement するのと似ている
- 文法：
  - トレイト定義：`trait` トレイト名 { シグネチャのみ or デフォルト実装 }
  - トレイト実装：`impl` トレイト名 `for` 構造体名 { 実装 }
  - ブランケット実装：`impl<T: トレイト名>` トレイト名 `for` `T` { 実装 }
- 制約：
  - コヒーレンス（_coherence_ ／ 孤児のルール（_orphan rule_））：外部のトレイトを外部の型に対して実装することはできない
    - ⇒ 他人のコードが自分のコードを壊したり、その逆ができないことを保証
    - この制約がなければ、2 つのクレートが同じ型に対して同じトレイトを実装できてしまい、 コンパイラはどちらの実装を使うべきかわからなくなってしまう
- 注意：デフォルト実装メソッドをオーバーライドしている実装からそのデフォルト実装メソッド呼び出すことはできない
- 引数としてのトレイト：
  - 文法：
    - 関数定義：
      - impl Trait 構文 1：`fn` 関数名(引数: ` &``impl ` Trait) { 実装 }
      - impl Trait 構文 2：`fn` 関数名(引数: `&`(`impl` Trait + Trait)) { 実装 }
      - トレイト境界構文：
        - 例 1：`fn` 関数名`<T:` Trait `>`(引数: `&T`) { 実装 }
        - 例 2：`fn` 関数名`<T:` Trait + Trait`>`(引数: `&T` , 引数: `&T`) { 実装 }
        - 例 3：`fn` 関数名`<T, U>`(引数: `&T`, 引数: `&U`) -> 戻り値型 `where` `T`: Trait + Trait, U: Trait + Trait { 実装 }
    - メソッド定義：
      - 例：`impl` `<T:` Trait1 `+` Trait2`>` 構造体名`<T>` { `fn` メソッド名(`&self`) { 実装 } }
        - この場合、Trait1 と Trait2 を実装している T 型のみがメソッドを使用することができる（トレイト境界によりメソッドの呼出し可否をコントロールする）
- 戻り値としてのトレイト
  - 文法：`fn` 関数名() -> `impl` トレイト名 { 実装 }
  - 利点：クロージャとイテレータの作り出す型は、コンパイラだけが知っているものであったり、指定するには長すぎるものであったりします。 impl Trait 構文を使えば、非常に長い型を書くことなく、ある関数は Iterator トレイトを実装するある型を返すのだ、と簡潔に指定することができます。
  - 制約：impl Trait は一種類の型を返す場合にのみ使える
- ブランケット実装（_blanket implementation_）：トレイト境界を満たすあらゆる型にトレイトを実装すること
  - 例：
    - Display トレイトを実装するあらゆる型に ToString トレイトを実装
      - `impl` `<T:` Display`>` ToString `for` `T` { --snip-- }
      - ⇒ Display トレイトを実装する任意の型が、 ToString トレイトで定義された to_string メソッドを呼び出せる
  - ブランケット実装はトレイトのドキュメンテーションの _Implementors_ 節に出現する

### -- code --

#### `impl Trait` 構文（impl Trait syntax）

トレイトの引数が１つの場合 ⭕

```rust
// ⭕
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// △ item1とitem2の型が（どちらもSummaryを実装する限り）異なっても良いとするならば、impl Traitは適切
pub fn notify(item1: &impl Summary, item2: &impl Summary) {/*--snip--*/}
```

#### トレイト境界構文（_Trait Bound Syntax_）

トレイトの引数が複数の場合 ⭕

```rust
// △ `impl Trait` 構文の方がシンプル
pub fn notify<T: Summary>(item: &T) {
    // 速報！ {}
    println!("Breaking news! {}", item.summarize());
}


// ⭕ 両方のトレイト引数が同じ型であることを強制するするにはトレイト境界構文を使う
pub fn notify<T: Summary>(item1: &T, item2: &T) {/*--snip--*/}
```

## ライフタイム

- 大要：
  - その参照が有効になるスコープ
  - コンパイラに「参照がお互いにどう関係しているか」の情報を与える、一種の（ライフタイム版の）ジェネリクス
    - ⇒ `<'a, T>`というように型引数`T`と同列に書く
- 他のプログラミング言語との比較：ライフタイムの概念は、他のプログラミング言語の道具とはどこか異なり、間違いなく、 Rust で一番際立った機能になっている
- 多くの場合、暗黙的に推論される
  - 類似：型が推論される
- 参照のライフタイムがいくつか異なる方法で関係することがある場合には注釈が必要
  - 類似：複数の型の可能性があるときには、型注釈が必要
- ライフタイム注釈
  - 注釈は、複数の参照のジェネリックなライフタイム引数が、 お互いにどう関係するかをコンパイラに指示することを意図している
  - 参照の生存期間も変えることは決してない
  - 関数へのライフタイム注釈が必要になる場面
    - 関数に関数外からの参照（引数で参照を受け取る）や関数外への参照（戻り値で参照を返す）がある場合
      - ∵ この場合、コンパイラが引数や戻り値のライフタイムを自力で解決することはほとんど不可能
        - ⇒ ライフタイム注釈は、関数の引数と戻り値のライフタイムを接続する
  - 記法：
    - ライフタイム引数の名前はアポストロフィー(')で始める
    - 通常すべて小文字で短く記載する（「'a」とすることが多い）
    - ライフタイム引数注釈は、参照の`&`の後に配置し、注釈と参照の型を区別するために空白を 1 つ使用する
    - 例：
      - `&i32` (ただの)参照
      - `&'a i32` 明示的なライフタイム付きの参照
      - `&'a mut i32` 明示的なライフタイム付きの可変参照
      - `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { --snip-- }`
        - このシグニチャで表現したい制約は、引数の全ての参照と戻り値が同じライフタイムを持つこと
        - このシグニチャを満たすようなスコープを`'a`に代入できるということをコンパイラに伝える
        - `'a`は`x`と`y`のうち、短いライフタイムを持つ方のライフタイムとなる
        - memo: 正直この場合は、コンパイラがライフタイムを推論してくれても良いのでは？と思ってしまう。「全ての引数を戻り値に使っている場合、戻り値のライフタイムは最短のものを使う」と推論してもよいのでは？もっと複雑になった場合の例が欲しい。
          - memo: と思ったら「将来的に、さらに少数のライフタイム注釈しか必要にならない可能性もあります。」と書いてあった。コンパイラの参照解析に落とし込まれたパターンは、ライフタイム省略規則というらしい。
- 入力ライフタイム：関数やメソッドの引数のライフタイム
- 出力ライフタイム：戻り値のライフタイム
- 明示的な注釈がない場合のライフタイム：
  - ルール：参照を受け取る引数はそれぞれ独立した個別のライフタイムをもつものと見做される
  - ルール：入力ライフタイム引数がたった一つの場合、そのライフタイムと出力ライフタイムは同一と見做される
  - ルール：メソッドの場合（引数の一つが`&self`や`&mut self`だった場合）、出力ライフタイムは`self`のライフタイムと見做す。
  - memo: _all output lifetime parameters_ が複数形になっているのは `return` が複数ある場合のことを踏まえたからか。_parameters_ と書いているのは、ジェネリックを引数に見立てているからか。
  - 上記の全てのルールを適用してもライフタイムが決定されない場合、ライフタイム注釈が必要になる。
- 静的ライフタイム
  - プログラム全体の期間
  - 文字列リテラルは全て'static ライフタイムになる
  - 注意：エラーメッセージで'static ライフタイムを使用する提言を目撃しても、安易に'static ライフタイムを使わない

## テスト

- 属性：Rust コードの部品に関するメタデータ
  - 例：derive 属性
  - `test`
    - 関数をテスト関数として扱う
  - `should_panic`
    - 関数内のコードがパニックしたら、テストを通過させ、パニックしなかったら失敗させる
    - should_panic のテストの正確を期すために、should_panic 属性に`expected`引数を追加することもできます。
  - `ignore`：テストをスキップする
  - `cfg`：この属性に続くアイテムが、ある特定の設定オプションを与えられたら、そのアイテムを含むように、コンパイラに指示
    - configuration の意味
    - 例：`#[cfg(test)]`：cargo test で積極的にテストを実行した場合のみ、 Cargo がテストコードをコンパイルする。
      - 注意：`#[cfg(test)]`注釈を付与したモジュールに含まれるヘルパー関数もコンパイルされる（#[test]で注釈された関数だけコンパイルされるのではない）
- テスト関数：test 属性で注釈された関数／fn の前に#[test]を付け加えた関数
  - cargo test コマンドでテストを実行したら、コンパイラは test 属性で注釈された関数を走らせるテスト用バイナリをビルドし、 各テスト関数が通過したか失敗したかを報告します。
  - `panic!`マクロの呼出しで FAIL する
  - `Result<T, E>.Err`の返却で FAIL する
    - 注意：`Result<T, E>`を使うテストに#[should_panic]注釈を使うことはできない
- マクロ
  - assert_eq!と assert_ne!
    - 比較対象の値は PartialEq と Debug トレイトを実装していなければなりません。
      - ∵ アサーションが失敗すると、これらのマクロは引数をデバッグフォーマットを使用してプリントする
    - 例：`assert_eq!(2 + 2, 4);`
      - 内部：`==`演算子を利用
    - `assert_ne!`： `assert_eq!`の逆
      - 内部：`!=`演算子を利用
  - 例：`assert!(larger.can_hold(&smaller));`
- cargo test
  - `cargo test some_test`： some_test という文字列を含む関数のみをテスト
  - デフォルト：
    - 複数スレッドを使用して並行処理する
    - FAIL した関数の標準出力のみ表示される
  - コマンドラインオプション
    - cargo test にかかるものと、出来上がったテストバイナリにかかるものとがある
      - cargo test にかかるオプション：
        - `--`という区分記号の後
        - 例：cargo test -- --help
        - `--test-threads`フラグ
          - `cargo test -- --test-threads=1`
            - テストスレッドの数を 1 にセットし、並行性を使用しないようにプログラムに指示
              - 書くプログラムが一つのファイルを扱う場合など、並列処理だと困る場合に使う
          - `cargo test -- --nocapture`
            - 出力キャプチャ機能（ok なケースの標準出力の抑制）を無効化
            - デフォルトではテストは並列で実行されるので標準出力はあべこべになる
          - `cargo test -- --ignored`
            - `ignored`属性のテスト関数のみを実行する
      - 出来上がったテストバイナリにかかるオプション
        - `--`という区分記号の前
        - 例：cargo test --help
        - `cargo test --test` ファイル名：特定の結合テストファイルにあるテストを全て走らせる
- 単体テスト
  - 目的：残りのコードから切り離して各単位のコードをテストし、 コードが想定通り、動いたり動いていなかったりする箇所を迅速に特定すること
  - 配置場所：テスト対象となるコードと共に、src ディレクトリの各ファイルに置く
    - 慣習：各ファイルに tests という名前のモジュールを作り、テスト関数を含ませ、 そのモジュールを `cfg(test)`で注釈する
  - 非公開関数をテストすることもできる（同じファイル内なら非公開関数を実行できるから）
- tests モジュールの#[cfg(test)]という注釈：コンパイラに cargo build を走らせた時ではなく、cargo test を走らせた時にだけ、 テストコードをコンパイルし走らせるよう指示する
  - ⇒ ライブラリをビルドしたいだけの時にはコンパイルタイムを節約
  - ⇒ テストが含まれないので、コンパイル後の成果物のサイズを節約
  - 単体テストを慣習通りにテスト対象と同じファイルにテストを書く場合に`#[cfg(test)]`が必要となる
    - ⇒ 結合テストは別のディレクトリに存在することになるので、`#[cfg(test)]`注釈は必要ない
- 結合テスト
  - 目的：ライブラリのいろんな部分が共同で正常に動作しているかをテストすること
    - ∵ 単体では正常に動くコードも、結合した状態だと問題を孕む可能性もある
  - ライブラリの公開 API を使用して外部コードが使用するのと同じ方法でコードをテストする
  - tests ディレクトリ
    - 結合テストの作成に必要
    - 場所：プロジェクトディレクトリのトップ階層（src の隣）
    - Cargo は tests ディレクトリ内のそれぞれのテストファイルを個別のクレートとしてコンパイルする
      - ⇒ テストファイルは、`extern crate`で、各々ライブラリをインポートする必要がある
      - 但し：テストディレクトリのサブディレクトリにあるファイルは、個別のクレートとしてコンパイルされたり、テスト出力にセクションが含まれたりすることはない。
    - Cargo は tests ディレクトリを特別に扱い、cargo test を走らせた時にのみこのディレクトリのファイルをコンパイルする
      - ⇒ `#[cfg(test)]`注釈は不要

## 言葉

### レジストリ

- レジストリ
  - Crates.io のデータのコピー
- Crates.io
  - Rust のエコシステムにいる人間が、 他の人が使えるように自分のオープンソースの Rust プロジェクトを投稿する場所

## リンク

- [キーワード一覧](https://doc.rust-lang.org/stable/book/appendix-01-keywords.html)
- [Rust Documentation（公式ドキュメント）](https://doc.rust-lang.org/stable/#the-rust-programming-language)
- [Docker Official Images](https://hub.docker.com/_/rust)
  - とりあえず docker で試す：`docker container run -it rust`
    - ディストリビューション名確認：`cat /etc/issue`
    - [version 確認](https://eng-entrance.com/linux-os-version)：`cat /etc/debian_version`

## temp memo

- 10.2.
  - クロージャとイテレータの作り出す型は、コンパイラだけが知っているものであったり、指定するには長すぎるものであったりします。

## 感想

- Rust
  - エラー文が分かりやすい
    - ⇒ コンパイルエラーになるパターンはわざわざ覚える必要なさそう
  - 「多くの言語では、 回復可能なエラーと回復不能なエラーの 2 種類のエラーを区別することはなく、例外などの機構を使用して同様に扱う。」
    - これを言語レベルで明確に区別しているのは非常にありがたい
  - `?`演算子が強力
    - 笑ってしまうくらい最高だった
  - 「Rust のエラー処理機能は、プログラマがより頑健なコードを書く手助けをするように設計されています。」に納得した
- The Book
  - 解説が丁寧で分かりやすい
- 訳
  - シグニチャよりもシグネチャの方が一般的か（少なくとも僕が所有している３本つの本は「シグネチャ」を使用している）
