#[derive(Debug,Clone,Default)]
pub struct Mine {
    pub neighbor_count: u32,
    pub is_bomb: bool,
    pub is_visible: bool,
}

// impl Mine {
//      pub fn new() -> Self {
//          Mine {neighbor_count:0,
//          is_bomb: false,
//          is_visible: false,}
//      }
//      pub fn new_bomb() -> Self {
//          let mut m = Mine::new();
//          m.is_bomb=true;
//          m
//      }
// }
