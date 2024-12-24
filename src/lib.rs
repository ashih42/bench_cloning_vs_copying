#[derive(Clone)]
struct CloneableVector2D {
    x: i32,
    y: i32,
}

impl CloneableVector2D {
    fn get_xy(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn bench_cloneable_vector2d_by_cloning() -> bool {
    let a = CloneableVector2D { x: 0, y: 0 };
    let b = a.clone();

    let (ax, ay) = a.get_xy();
    let (bx, by) = b.get_xy();

    ax == bx && ay == by
}

#[derive(Clone, Copy)]
struct CopyableVector2D {
    x: i32,
    y: i32,
}

impl CopyableVector2D {
    fn get_xy(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub fn bench_copyable_vector2d_by_cloning() -> bool {
    let a = CopyableVector2D { x: 0, y: 0 };
    #[allow(clippy::clone_on_copy)]
    let b = a.clone();

    let (ax, ay) = a.get_xy();
    let (bx, by) = b.get_xy();

    ax == bx && ay == by
}

pub fn bench_copyable_vector2d_by_copying() -> bool {
    let a = CopyableVector2D { x: 0, y: 0 };
    let b = a;

    let (ax, ay) = a.get_xy();
    let (bx, by) = b.get_xy();

    ax == bx && ay == by
}
