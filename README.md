# Rustplacements

This is a compiler plugin for the [Rust language](https://www.rust-lang.org/en-US/) that replaces all of your string literals 
in the source code with random text. Well, it's not really random. You can choose to replace text with items from any of the 
lists on [this page](https://github.com/Peternator7/rustplacements/blob/master/CATEGORIES.md) by simply adding a few 
attributes to your existing Rust code.

## A Brief Example

Let's start with a simple example like the one below. It prints out the words in the sentence below one word at a time.

```rust
const SENTENCE: [&'static str; 9] = ["The", "Quick", "Brown", "Fox", "Jumped", "Over", "the",
                                     "Lazy", "Dog"];

fn main() {
    for word in &SENTENCE {
        println!("{}", word);
    }
}
```

The output should look like:

```txt
The
Quick
Brown
Fox
Jumped
Over
the
Lazy
Dog
```

Rustplacements let's us replace all the strings at compile with other values. Let's say we want to replace all the text with
emojis. Rustplacements can do that.

```rust
#![feature(plugin)]
#![plugin(rustplacements)]

// Placing it in the module root will replace everything in the module
#![Rustplacements = "emojis"]

const SENTENCE: [&'static str; 9] = ["The", "Quick", "Brown", "Fox", "Jumped", "Over", "the",
                                     "Lazy", "Dog"];

fn main() {
    for word in &SENTENCE {
        println!("{}", word);
    }
}
```

The new output will look something like this. The output is randomized so it will be re-generated everytime you compile 
your crate.

```text
😢 😫 🤓
😞 😠 😟 😖 😧
😬 😬 😈 😡 😟
😓 😒 😬
😝 😘 🤧 😬 😧 😡
😗 😈 😉 😫
😄 😱 😰
😃 🤡 😅 😯
🤒 😈 😈
```

## Using Rustplacements

Compiler plugins like Rustplacements are only available on nightly rust because they require a feature flag to use. To get started,
Rustplacements is available on [crates.io](https://crates.io/crates/rustplacements). To download the latest version, add the 
following line to the `Cargo.toml`.

```
[dependencies]
rustplacements = "*"
```

To enable the compiler plugin, add the following lines on the top of your `main.rs` or `lib.rs`. 

```
#![feature(plugin)]
#![plugin(rustplacements)]
```

You can now use the plugin anywhere in the crate by applying the `#[Rustplacements = "one-direction"]` to any language element.
You can place the element in the root with `#![Rustplacements = "got-quotes"]` and it will transform all the string literals
in your module. It can also be applied to specific strings / impls / functions for more fine grained control.

That's pretty much all there is to it. Check out the [categories page](https://github.com/Peternator7/rustplacements/blob/master/CATEGORIES.md) for more categories that you can use.

## Contributing

Do you have a category that you'd like to add? Feel free to open a PR with your new category, or edits to an existing category.
You'll need to change `exprs.rs` and the `CATEGORIES.md` file to include the new category. See the contributing page for 
more details. All Pull Requests are expected to be professional, inclusive of others, and generally non-offensive. We'll err
on the side of more conservative if there's any debate about the appropriateness of a category.
