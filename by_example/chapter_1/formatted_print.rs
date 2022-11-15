// rustc formatted_print.rs -o a.out  && ./a.out

/**
 * format!: write formatted text to String
 * print!: same as format! but the text is printed to the console (io::stdout).
 * println!: same as print! but a newline is appended.
 * eprint!: same as print! but the text is printed to the standard error (io::stderr).
 * eprintln!: same as eprint! but a newline is appended.
 */

fn main() {
    println!("{} days", 21);
    println!("{} days, {} months", 21, 12);

    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}.",
        subject = "I",
        verb = "like",
        object = "turtles",
    );

    println!("Base 10:   {}", 69420);
    println!("Base 2:    {:b}", 69420);
    println!("Base 8:    {:o}", 69420);
    println!("Base 16:   {:x}", 69420);
    println!("Base 16:   {:X}", 69420);

    println!("{number:>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(42));

    // https://doc.rust-lang.org/std/fmt/#formatting-traits
    // https://doc.rust-lang.org/std/primitive.tuple.html
    // https://doc.rust-lang.org/std/keyword.struct.html
    // impl std::fmt::Display for Structure {
    //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         // The `f` value implements the `Write` trait, which is what the
    //         // write! macro is expecting. Note that this formatting ignores the
    //         // various flags provided to format strings.
    //         write!(f, "Structure({})", self.0)
    //     }
    // }
    // println!("This struct `{}` will now print!", Structure(42));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // https://doc.rust-lang.org/std/fmt/#precision
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}
