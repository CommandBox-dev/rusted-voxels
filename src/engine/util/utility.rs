
#[inline]
pub fn inside_circle(dx: i32, dy: i32, r: i32) -> bool {
    (dx * dx + dy * dy <= (r * r) as i32)
}