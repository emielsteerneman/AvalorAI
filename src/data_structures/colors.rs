#[derive(Hash, PartialEq, Eq)]
pub struct Color(pub u32);

#[allow(dead_code)]
impl Color {
    pub const RED: Color = Color(0xFF0000);
    pub const GREEN: Color = Color(0x00FF00);
    pub const BLUE: Color = Color(0x0000FF);
    pub const YELLOW: Color = Color(0xFFFF00);
    pub const CYAN: Color = Color(0x00FFFF);
    pub const MAGENTA: Color = Color(0xFF00FF);
    pub const WHITE: Color = Color(0xFFFFFF);
    pub const BLACK: Color = Color(0x000000);
}
