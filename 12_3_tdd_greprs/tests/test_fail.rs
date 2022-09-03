use greprs::my_greprs::*;

#[test]
fn search_on_contents_test() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let config = GrepConfigs::new_config(
        query, "1234"
    ).unwrap();

    let match_vec = config.search_on_contents(contents);

    assert_eq!(match_vec.len(), 1);
}