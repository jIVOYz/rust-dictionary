use dictionary::*;

fn new_dictionary() -> Dictionary {
    return Dictionary::new();
}

fn dictionary_with_words() -> Dictionary {
    let mut dictionary = new_dictionary();
    let word_1 = Word::new(String::from("simple"), vec![String::from("просто")], None);
    let word_2 = Word::new(String::from("fire"), vec![String::from("огонь")], None);
    let word_3 = Word::new(String::from("fish"), vec![String::from("рыба")], None);
    let word_4 = Word::new(String::from("ball"), vec![String::from("мяч")], None);
    let word_5 = Word::new(String::from("cat"), vec![String::from("кот")], None);
    dictionary.add_word(word_1);
    dictionary.add_word(word_2);
    dictionary.add_word(word_3);
    dictionary.add_word(word_4);
    dictionary.add_word(word_5);

    return dictionary
}

#[test]
#[should_panic(expected = "no such word")]
fn remove_nonexisting_word() {
    let mut dictionary = dictionary_with_words();

    dictionary.remove_word(&[100]).expect("no such word");
}
