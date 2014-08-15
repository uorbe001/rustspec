# Rustspec

Nice syntax sugar and errors for bdd testing in rust (similar to rspec or chai).

I find the errors rust's built-in assert! gives pretty limited, and I personally like this sort of syntax better so I decided to start this as learning exercise.

## Usage

Add this as a dependency to your `Cargo.toml` and run `cargo build`:

```
[dependencies.rustspec_assertions]
git = "https://github.com/uorbe001/rustspec-assertions.git"

[dependencies.rustspec]
git = "https://github.com/uorbe001/rustspec.git"
```

Now you should be able to use these assertions in your tests by loading the cargo:

```
#[phase(plugin, link)] extern crate rustspec;
#[phase(plugin)] extern crate rustspec_assertions;

#[deriving(Show)]
#[deriving(Clone)]
#[deriving(PartialEq)]
struct Point {
    x: int,
    y: int
}

impl Add<Point, Point> for Point {
    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

scenario!("Point", {
    before({
        let one = 1i;
    });

    describe("#add", {
        before({
            let point_a = ::Point { x: one, y: one };
            let point_b = ::Point { x: 2i, y: 2i };
        });

        it("adds two points", {
            let point_c = point_a + point_b;
            expect(point_c.x).to(eq!(3i));
            expect(point_c.y).to(eq!(3i));
        });

        it.fails("adds two points and fails", {
            let point_c = point_a + point_b;
            expect(point_c.x).to(eq!(4i));
            expect(point_c.y).to(eq!(4i));
        });

        it.ignores("ignores this", {
            let point_c = point_a + point_b;
            expect(point_c.x).to(eq!(4i));
            expect(point_c.y).to(eq!(4i));
        });

        context("testing PartialEq", {
            before({
                let point_3 = point_a + point_b;
            });

            it("passes with equals", {
                let point_c = point_a + point_b;
                expect(point_c).to(eq!(point_3));
            });
        });
    });
})
```

The crate relies on macros, so you'll need to add this to your test.rs, lib.rs or main.rs file:

```
#![feature(phase)]
```

For a complete list of matchers and more examples, please check the [assertion tests](https://github.com/uorbe001/rustspec-assertions/tree/master/tests) and for syntax examples check the [tests](tests/).

## Collaborating

If you want to help build this up, feel free to open a PR on this repo or the [assertions repository](https://github.com/uorbe001/rustspec-assertions) and I'll try to check it out as soon as possible.

## Work in progress

Please be aware this is work in progress and I'm a total rust newbie, so expect bugs .
