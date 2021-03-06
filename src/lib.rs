#![crate_type = "dylib"]
#![feature(plugin_registrar, rustc_private)]

//! # Rustplacements
//!
//! This is a compiler plugin for the [Rust language](https://www.rust-lang.org/en-US/) that replaces all of your string literals
//! in the source code with random text. Well, it's not really random. You can choose to replace text with items from any of the
//! lists on [this page](https://github.com/Peternator7/rustplacements/blob/master/CATEGORIES.md) by simply adding a few
//! attributes to your existing Rust code.
//!
//! ## A Brief Example
//!
//! Let's start with a simple example like the one below. It prints out the words in the sentence below one word at a time.
//!
//! ```rust,ignore
//! const SENTENCE: [&'static str; 9] = ["The", "Quick", "Brown", "Fox", "Jumped", "Over", "the",
//!                                      "Lazy", "Dog"];
//!
//! fn main() {
//!     for word in &SENTENCE {
//!         println!("{}", word);
//!     }
//! }
//! ```
//!
//! The output should look like:
//!
//! ```txt
//! The
//! Quick
//! Brown
//! Fox
//! Jumped
//! Over
//! the
//! Lazy
//! Dog
//! ```
//!
//! Rustplacements let's us replace all the strings at compile with other values. Let's say we want to replace all the text with
//! emojis. Rustplacements can do that.
//!
//! ```rust,ignore
//! #![feature(plugin)]
//! #![plugin(rustplacements)]
//!
//! // Placing it in the module root will replace everything in the module
//! #![Rustplacements = "emojis"]
//!
//! const SENTENCE: [&'static str; 9] = ["The", "Quick", "Brown", "Fox", "Jumped", "Over", "the",
//!                                      "Lazy", "Dog"];
//!
//! fn main() {
//!     for word in &SENTENCE {
//!         println!("{}", word);
//!     }
//! }
//! ```
//!
//! The new output will look something like this. The output is randomized so it will be re-generated everytime you compile
//! your crate.
//!
//! ```text
//! 😢 😫 🤓
//! 😞 😠 😟 😖 😧
//! 😬 😬 😈 😡 😟
//! 😓 😒 😬
//! 😝 😘 🤧 😬 😧 😡
//! 😗 😈 😉 😫
//! 😄 😱 😰
//! 😃 🤡 😅 😯
//! 🤒 😈 😈
//! ```
//!
//! ## Using Rustplacements
//!
//! Compiler plugins like Rustplacements are only available on nightly rust because they require a feature flag to use. To get started,
//! Rustplacements is available on [crates.io](https://crates.io/crates/rustplacements). To download the latest version, add the
//! following line to the `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! rustplacements = "*"
//! ```
//!
//! To enable the compiler plugin, add the following lines on the top of your `main.rs` or `lib.rs`.
//!
//! ```rust,ignore
//! #![feature(plugin)]
//! #![plugin(rustplacements)]
//! ```
//!
//! You can now use the plugin anywhere in the crate by applying the `#[Rustplacements = "one-direction"]` to any language element.
//! You can place the element in the root with `#![Rustplacements = "got-quotes"]` and it will transform all the string literals
//! in your module. It can also be applied to specific strings / impls / functions for more fine grained control.
//!
//! That's pretty much all there is to it. Check out the [categories page](https://github.com/Peternator7/rustplacements/blob/master/CATEGORIES.md) for more categories that you can use.

extern crate syntax;
extern crate rustc_plugin;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use rustc_plugin::Registry;
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::ast::*;
use syntax::codemap::Span;
use syntax::symbol::Symbol;
use syntax::codemap::Spanned;
use syntax::ptr::P;

mod exprs;

struct Context<'a> {
    text: &'a Vec<&'static str>,
}

/// Compiler hook for Rust to register plugins.
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(Symbol::intern("Rustplacements"),
                                  SyntaxExtension::MultiModifier(Box::new(rustplace)))
}

fn rustplace(_: &mut ExtCtxt, _: Span, m: &MetaItem, an: Annotatable) -> Vec<Annotatable> {
    let category = match m.node {
        MetaItemKind::List(..) => panic!("This plugin does not support list style attributes."),
        MetaItemKind::Word => Symbol::intern("fizzbuzz"),
        MetaItemKind::NameValue(ref l) => {
            use LitKind::*;
            match l.node {
                Str(symbol, _) => symbol,
                _ => panic!("Only string literals are supported"),
            }
        }
    };

    let ctxt = Context { text: exprs::HASHMAP.get(&*category.as_str()).unwrap() };
    vec![an.trans(&ctxt)]
}

trait Rustplace {
    fn trans(self, ctxt: &Context) -> Self;
}

impl<T: Rustplace + 'static> Rustplace for P<T> {
    fn trans(self, ctxt: &Context) -> Self {
        self.map(|inner| inner.trans(ctxt))
    }
}

impl<T: Rustplace> Rustplace for Vec<T> {
    fn trans(self, ctxt: &Context) -> Self {
        self.into_iter().map(|i| i.trans(ctxt)).collect()
    }
}

// We can invoke this rule on most of the struct types.
macro_rules! Rustplace {
    // For many of the structs, the field is called "node" so we simplify that case.
    ($ty:ident) => (Rustplace!($ty,node););
    ($ty:ident,$field:tt) => (
        impl Rustplace for $ty {
            fn trans(self, ctxt: &Context) -> Self {
                $ty {
                    $field: self.$field.trans(ctxt),
                    ..self
                }
            }
        }
    )
}

// We can autoimplement some of the structs because the all change the same field. :)
Rustplace!(Item);
Rustplace!(TraitItem);
Rustplace!(ImplItem);
Rustplace!(Stmt);
Rustplace!(Expr);
// These follow the same basic pattern, but the field has a different name.
Rustplace!(Block, stmts);
Rustplace!(Field, expr);
Rustplace!(Mod, items);

// These need 1 extra map so we just wrote them out.
impl Rustplace for Local {
    fn trans(self, ctxt: &Context) -> Self {
        Local {
            init: self.init.map(|i| i.trans(ctxt)),
            ..self
        }
    }
}

impl Rustplace for Arm {
    fn trans(self, ctxt: &Context) -> Self {
        Arm {
            guard: self.guard.map(|i| i.trans(ctxt)),
            ..self
        }
    }
}

// All the enums need to be manually implemented and we figure out what variants it makes sense
// for us to transform.
impl Rustplace for Annotatable {
    fn trans(self, ctxt: &Context) -> Self {
        use Annotatable::*;
        match self {
            Item(item) => Item(item.trans(ctxt)),
            TraitItem(item) => TraitItem(item.trans(ctxt)),
            ImplItem(item) => ImplItem(item.trans(ctxt)),
        }
    }
}

impl Rustplace for ItemKind {
    fn trans(self, ctxt: &Context) -> Self {
        use ItemKind::*;
        match self {
            Fn(a, b, c, d, e, block) => Fn(a, b, c, d, e, block.trans(ctxt)),
            Static(ty, m, expr) => Static(ty, m, expr.trans(ctxt)),
            Const(ty, expr) => Const(ty, expr.trans(ctxt)),
            Trait(u, g, ty, v) => Trait(u, g, ty, v.trans(ctxt)),
            Impl(a, b, c, d, e, f, v) => Impl(a, b, c, d, e, f, v.trans(ctxt)),
            Mod(m) => Mod(m.trans(ctxt)),
            _ => self,
        }
    }
}

impl Rustplace for TraitItemKind {
    fn trans(self, ctxt: &Context) -> Self {
        use TraitItemKind::*;
        match self {
            Const(ty, Some(expr)) => Const(ty, Some(expr.trans(ctxt))),
            Method(sig, Some(block)) => Method(sig, Some(block.trans(ctxt))),
            _ => self,
        }
    }
}

impl Rustplace for ImplItemKind {
    fn trans(self, ctxt: &Context) -> Self {
        use ImplItemKind::*;
        match self {
            Const(ty, expr) => Const(ty, expr.trans(ctxt)),
            Method(sig, block) => Method(sig, block.trans(ctxt)),
            _ => self,
        }
    }
}

impl Rustplace for StmtKind {
    fn trans(self, ctxt: &Context) -> Self {
        use StmtKind::*;
        match self {
            Local(l) => Local(l.trans(ctxt)),
            Item(i) => Item(i.trans(ctxt)),
            Expr(e) => Expr(e.trans(ctxt)),
            Semi(s) => Semi(s.trans(ctxt)),
            _ => self,
        }
    }
}

impl Rustplace for ExprKind {
    fn trans(self, ctxt: &Context) -> Self {
        use ExprKind::*;
        match self {
            Lit(l) => Lit(l.trans(ctxt)),
            Box(b) => Box(b.trans(ctxt)),
            InPlace(a, b) => InPlace(a.trans(ctxt), b.trans(ctxt)),
            Array(v) => Array(v.trans(ctxt)),
            Call(a, v) => Call(a.trans(ctxt), v.trans(ctxt)),
            MethodCall(p, v) => MethodCall(p, v.trans(ctxt)),
            Tup(v) => Tup(v.trans(ctxt)),
            Binary(op, l, r) => Binary(op, l.trans(ctxt), r.trans(ctxt)),
            Unary(op, expr) => Unary(op, expr.trans(ctxt)),
            Cast(expr, ty) => Cast(expr.trans(ctxt), ty),
            Type(expr, ty) => Type(expr.trans(ctxt), ty),
            If(cond, iff, els) => {
                If(cond.trans(ctxt),
                   iff.trans(ctxt),
                   els.map(|i| i.trans(ctxt)))
            }
            IfLet(pat, expr, iff, els) => {
                IfLet(pat,
                      expr.trans(ctxt),
                      iff.trans(ctxt),
                      els.map(|i| i.trans(ctxt)))
            }
            While(cond, blk, si) => While(cond.trans(ctxt), blk.trans(ctxt), si),
            WhileLet(p, expr, blk, si) => WhileLet(p, expr.trans(ctxt), blk.trans(ctxt), si),
            ForLoop(p, expr, blk, si) => ForLoop(p, expr.trans(ctxt), blk.trans(ctxt), si),
            Loop(expr, si) => Loop(expr.trans(ctxt), si),
            Match(expr, v) => Match(expr.trans(ctxt), v.trans(ctxt)),
            Closure(c, p, blk, s) => Closure(c, p, blk.trans(ctxt), s),
            Block(blk) => Block(blk.trans(ctxt)),
            Catch(blk) => Catch(blk.trans(ctxt)),
            Assign(a, b) => Assign(a.trans(ctxt), b.trans(ctxt)),
            AssignOp(op, lhs, rhs) => AssignOp(op, lhs.trans(ctxt), rhs.trans(ctxt)),
            Field(expr, si) => Field(expr.trans(ctxt), si),
            TupField(expr, span) => TupField(expr.trans(ctxt), span),
            Index(a, b) => Index(a.trans(ctxt), b.trans(ctxt)),
            Range(lower, upper, lim) => {
                Range(lower.map(|i| i.trans(ctxt)),
                      upper.map(|i| i.trans(ctxt)),
                      lim)
            }
            AddrOf(m, expr) => AddrOf(m, expr.trans(ctxt)),
            Break(br, expr) => Break(br, expr.map(|i| i.trans(ctxt))),
            Ret(opt) => Ret(opt.map(|i| i.trans(ctxt))),
            Struct(p, v, opt) => Struct(p, v.trans(ctxt), opt.map(|i| i.trans(ctxt))),
            Repeat(a, b) => Repeat(a.trans(ctxt), b.trans(ctxt)),
            Paren(expr) => Paren(expr.trans(ctxt)),
            Try(expr) => Try(expr.trans(ctxt)),
            _ => self,
        }
    }
}

impl Rustplace for Spanned<LitKind> {
    fn trans(self, ctxt: &Context) -> Self {
        use LitKind::*;
        match self.node {
            // All that code above just so we can do this one transformation :)
            Str(s, _) => {
                let new_string = s.as_str()
                    .lines()
                    .map(|line| {
                        let mut output = String::new();
                        let mut idx = 0;
                        // Copy the lead whitespace over.
                        for c in line.chars() {
                            if c.is_whitespace() {
                                idx += 1;
                                output.push(c);
                            } else {
                                break;
                            }
                        }

                        let l = line.chars().count();
                        // Now just append random stuff.
                        while idx < l {
                            let r = rand::random::<usize>() % ctxt.text.len();
                            output.push_str(ctxt.text[r]);
                            output.push(' ');
                            idx += ctxt.text[r].chars().count();
                        }

                        // TODO: Remove the trailing ' '.
                        output
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                Spanned {
                    node: LitKind::Str(Symbol::intern(&*new_string), StrStyle::Cooked),
                    ..self
                }
            }
            _ => self,
        }
    }
}
