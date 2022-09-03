use greprs::my_greprs::*;

fn file_init(query:&str) -> GrepConfigs {
    let contents = "resource/poem.txt";

    GrepConfigs::new_config(
        query, contents
    ).unwrap()
}

#[test]
fn file_test() {
    let mut config = file_init("body");

    let match_vec = config.search();

    assert_eq!(match_vec.len(), 3);
}

#[test]
fn nocase_test() {
    let mut config = file_init("BoDy");

    let match_vec = config.search_nocase();

    assert_eq!(match_vec.len(), 3);
}