#![feature(plugin, core)]
#[plugin] #[macro_use] extern crate rustspec;
#[macro_use] extern crate rustspec_assertions;

use std::ops::Add;

#[derive(Show)]
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
