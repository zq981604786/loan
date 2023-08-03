mod cam;
mod utils;

use crate::cam::index::App;

// trunk serve
fn main() {
    yew::Renderer::<App>::new().render();
}
