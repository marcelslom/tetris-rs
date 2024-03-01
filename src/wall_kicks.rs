use rusttype::Point;

pub static WALL_KICKS : [[Point<i32>; 5]; 8] = [
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: -1}, Point {x: 0, y: 2}, Point {x: -1, y: 2} ],
    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: 1}, Point {x: 0, y: -2}, Point {x: 1, y: -2} ],

    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: 1}, Point {x: 0, y: -2}, Point {x: 1, y: -2} ],
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: -1}, Point {x: 0, y: 2}, Point {x: -1, y: 2} ],

    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: -1}, Point {x: 0, y: 2}, Point {x: 1, y: 2} ],
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: 1}, Point {x: 0, y: -2}, Point {x: -1, y: -2} ],

    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: -1, y: 1}, Point {x: 0, y: -2}, Point {x: -1, y: -2} ],
    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 1, y: -1}, Point {x: 0, y: 2}, Point {x: 1, y: 2} ],
];

pub static I_WALL_KICKS : [[Point<i32>; 5]; 8] = [
    [Point {x: 0, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 1}, Point {x: 1, y: -2} ],
    [Point {x: 0, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: -1}, Point {x: -1, y: 2} ],

    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: -2}, Point {x: 2, y: 1} ],
    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 2}, Point {x: -2, y: -1} ],

    [Point {x: 0, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: -1}, Point {x: -1, y: 2} ],
    [Point {x: 0, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 1}, Point {x: 1, y: -2} ],

    [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: -2, y: 0}, Point {x: 1, y: 2}, Point {x: -2, y: -1} ],
    [Point {x: 0, y: 0}, Point {x: -1, y: 0}, Point {x: 2, y: 0}, Point {x: -1, y: -2}, Point {x: 2, y: 1} ],
];