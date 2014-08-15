#![feature(phase)]

#[phase(plugin, link)] extern crate rustspec;
#[phase(plugin, link)] extern crate rustspec_assertions;

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
    when("#add", {
        before({
            let point_a = ::Point { x: 1i, y: 1i };
            let point_b = ::Point { x: 2i, y: 2i };
        });

        it("adds two points", {
            let point_c = point_a + point_b;
            expect(point_c.x).to(eq!(3i));
            expect(point_c.y).to(eq!(3i));
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


//         it fails!(adds_two_points_wrong {
//             let point_c = point_a + point_b;
//             expect(point_c.x).to(eq!(4i));
//             expect(point_c.y).to(eq!(4i));
//         })

//         it ignores!(this_is_ignored {
//             expect(1i).not_to(eq!(2i));
//         })

