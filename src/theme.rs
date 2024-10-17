#[allow(unused)]
#[derive(Copy, Clone)]
pub struct Theme<'a> {
    pub primary_color: &'a str,
    pub secondary_color: &'a str,
    pub background_color: &'a str,
    pub text_color: &'a str,
    pub accent_color: &'a str,
    pub border_color: &'a str,
}
