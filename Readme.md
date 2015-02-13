# Rustspec ![build status](https://travis-ci.org/uorbe001/rustspec.svg?branch=master)

Nice syntax sugar and errors for bdd testing in rust (similar to rspec or chai).

I find the errors rust's built-in assert! gives pretty limited, and I personally like this sort of syntax better so I decided to start this as learning exercise.

## Usage

Add this as a dependency to your `Cargo.toml` and run `cargo build`:

```
[dependencies]
rustspec_assertions = "~0.1.4"
rustspec = "~0.1.3"
```

Now you should be able to use these assertions in your tests by loading the cargo:

```
#![feature(plugin)]
#![plugin(rustspec, rustspec_assertions)]
#[macro_use] extern crate rustspec;
#[macro_use] extern crate rustspec_assertions;

use std::ops::Add;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Point {
    x: isize,
    y: isize
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

scenario!("Point", {
    before({
        let one = 1is;
    });

    describe("#add", {
        before({
            let point_a = ::Point { x: one, y: one };
            let point_b = ::Point { x: 2is, y: 2is };
        });

        it("adds two points", {
            let point_c = point_a + point_b;
            expect(&point_c.x).to(eq!(3is));
            expect(&point_c.y).to(eq!(3is));
        });

        it.fails("adds two points and fails", {
            let point_c = point_a + point_b;
            expect(&point_c.x).to(eq!(4is));
            expect(&point_c.y).to(eq!(4is));
        });

        it.ignores("ignores this and something CAPITALIZED", {
            let point_c = point_a + point_b;
            expect(&point_c.x).to(eq!(4is));
            expect(&point_c.y).to(eq!(4is));
        });

        // There is a bug on rustc's hygien checking preventing this
	// from working for now, point_3 is not defined on the 'expect'
	// line because of it... essentially, it means you can use
	// variables defined on before blocks on the expect(), but not
	// on the eq!()
         // context("testing PartialEq", {
             // before({
             //     let point_3 = point_a + point_b;
             // });

             // it("passes with equals", {
             //     let point_c = point_a + point_b;
             //     expect(&point_c).to(eq!(point_3));
             // });
         // });
    });

    describe("a block without before", {
        it("works", {
            expect(&false).not_to(be_true!());
        });
    });
});
```

The crate relies on macros, so you'll need to add this to your test.rs, lib.rs or main.rs file:

```
#![feature(plugin)]
#![plugin(rustspec, rustspec_assertions)]
#[macro_use] extern crate rustspec;
#[macro_use] extern crate rustspec_assertions;
```

For a complete list of matchers and more examples, please check the [assertion tests](https://github.com/uorbe001/rustspec-assertions/tree/master/tests) and for syntax examples check the [tests](tests/).

### BUG

There is an issue with the latest rust nighly build and the hygiene checking, so referencing a variable inside one of the matchers (eq!, be_gt!, etc) won't work with rust versions that are after [this PR](https://github.com/rust-lang/rust/pull/16477) was merged (which is why the build is failing at the moment). I'll remove this note as soon as I see it works again, currently tracking the issue [here](https://github.com/rust-lang/rust/issues/8063).

## Collaborating

If you want to help build this up, feel free to open a PR on this repo or the [assertions repository](https://github.com/uorbe001/rustspec-assertions) and I'll try to check it out as soon as possible.

## Work in progress

Please be aware this is work in progress and I'm a total rust newbie, so expect bugs .

### TODO

* Find a way to get rid of the assertions dependency for clients.
* Improve failed assertion line reporting (trying to find out a way to do this, having issues apparently related to [15962](https://github.com/rust-lang/rust/issues/15962) and [16472](https://github.com/rust-lang/rust/issues/16472)).
* Add after
