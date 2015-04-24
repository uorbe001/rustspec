#![feature(plugin)]
#![plugin(rustspec, rustspec_assertions)]
#![allow(plugin_as_library)]
extern crate rustspec;
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
        let one = 1isize;
    });

    describe("#add", {
        before({
            let point_a = ::Point { x: one, y: one };
            let point_b = ::Point { x: 2isize, y: 2isize };
        });

        it("adds two points", {
            let point_c = point_a + point_b;
            expect(&point_c.x).to(eq!(3isize));
            expect(&point_c.y).to(eq!(3isize));
        });

        it.fails("adds two points and fails", {
            let point_c = point_a + point_b;
            expect(&point_c.x).to(eq!(4isize));
            expect(&point_c.y).to(eq!(4isize));
        });

        it.ignores("ignores this and something CAPITALIZED", {
            let point_c = point_a + point_b;
            expect(&point_c.x).to(eq!(4isize));
            expect(&point_c.y).to(eq!(4isize));
        });

        // Commented until bugfixed
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
