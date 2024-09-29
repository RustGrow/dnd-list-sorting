#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub id: usize,
    pub board_list_id: usize,
    pub title: &'static str,
    pub color: &'static str,
}

impl Card {
    pub fn new(id: usize, board_list_id: usize, title: &'static str, color: &'static str) -> Self {
        Self {
            id,
            board_list_id,
            title,
            color,
        }
    }
}
