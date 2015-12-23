Gimei
====

[gimei](https://github.com/willnet/gimei) のRust移植版です。漢字、ひらがな、カタカナの3種類の情報を持った架空の情報を生成します。

## 使い方

### 名前をランダムに返す

下記のように使います

```rust
let gimei = gimei::name();
gimei.kanji()           // => 斎藤 陽菜
gimei.hiragana()        // => さいとう はるな
gimei.katakana()        // => サイトウ ハルナ
gimei.last.kanji        // => 斎藤
gimei.last.hiragana     // => さいとう
gimei.last.katakana     // => サイトウ
gimei.first.kanji       // => 陽菜
gimei.first.hiragana    // => はるな
gimei.first.katakana    // => ハルナ
```

下記のように男性／女性の名前を返すことを明示的に指定できます。`gimei::name()` の場合は男女の名前を等確率で返します。

```rust
let gimei = gimei::male();
gimei.is_male()         // => true
gimei.is_female()       // => false
gimei.kanji()           // => 小林 顕士

gimei = gimei::female();
gimei.is_male()         // => false
gimei.is_female()       // => true
gimei.kanji()           // => 根本 彩世
```

出力される名前の候補となるデータは `data/names.toml` にあるので、必要であればファイルを修正してください。

### 住所をランダムに返す

都道府県、区、市、町を組み合わせた住所情報を漢字、ひらがな、カタカナで取得することができます。

```rust
let address = gimei::address();
address.kanji()                // => 岡山県大島郡大和村稲木町
address.hiragana()             // => おかやまけんおおしまぐんやまとそんいなぎちょう
address.katakana()             // => オカヤマケンオオシマグンヤマトソンイナギチョウ

address.prefecture.kanji       // => 岡山県
address.prefecture.hiragana    // => おかやまけん
address.prefecture.katakana    // => オカヤマケン

address.city.kanji             // => 大島郡大和村
address.city.hiragana          // => おおしまぐんやまとそん
address.city.katakana          // => オオシマグンヤマトソン

address.town.kanji             // => 稲木町
address.town.hiragana          // => いなぎちょう
address.town.katakana          // => イナギチョウ
```

出力される住所の候補となるデータは `data/addresses.toml` にあるので、必要であればファイルを修正してください。
