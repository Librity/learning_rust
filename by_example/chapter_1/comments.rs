// rustc comments.rs -o a.out  && ./a.out

//! Another documentation comment (parsed into HTML)
fn printx() {
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

/// Documentation comment (parsed into HTML)
fn main() {
    // Line comment
    // println!("I am a comment!");

    /*
    Multi
    line
    comment
    */

    /**
     * Purdy
     * comment
     */
    printx();
}
