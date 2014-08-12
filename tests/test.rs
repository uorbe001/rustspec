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

scenario!(Point {
    test when!(adding {
        it lets!(point_a: ::Point -> ::Point { x: 1i, y: 1i })
        it lets!(point_b: ::Point -> ::Point { x: 2i, y: 2i })

        it passes!(adds_two_points {
            let point_c = point_a + point_b;
            expect(point_c.x).to(eq!(3i));
            expect(point_c.y).to(eq!(3i));
        })

        it fails!(adds_two_points_wrong {
            let point_c = point_a + point_b;
            expect(point_c.x).to(eq!(4i));
            expect(point_c.y).to(eq!(4i));
        })

        it ignores!(this_is_ignored {
            expect(1i).not_to(eq!(2i));
        })

        test when!(some_nested_special_case {
            it lets!(point_3: ::Point -> ::Point { x: 3i, y: 3i })

            it passes!(compares_two_points {
                let point_c = super::point_a + super::point_b;
                expect(point_c).to(eq!(point_3));
            })

            it fails!(compare_two_points_bad {
                let mut point_c = super::point_a + super::point_b;
                point_c.x += 1i;
                expect(point_c).to(eq!(point_3));
            })
        })
    })
})
