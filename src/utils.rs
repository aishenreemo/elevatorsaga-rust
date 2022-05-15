pub fn find_center_point(size: (u32, u32), offsets: (u32, u32)) -> (u32, u32) {
    (size.0 / 2 + offsets.0, size.1 / 2 + offsets.1)
}
