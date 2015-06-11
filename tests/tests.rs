extern crate gimei;

#[test]
fn tests() {
    let gimei = gimei::name();
    if gimei.is_female() {
        println!("性別: 女");
    } else {
        println!("性別: 男");
    }
    println!("漢字: {}", gimei.kanji());
    println!("ひらがな: {}", gimei.hiragana());
    println!("カタカナ: {}", gimei.katakana());
}
