mod mod_keyword;

fn main() {
    #[path="ref.rs"]
    mod ref_keyword;

    ref_keyword::run();
    mod_keyword::run();
}
