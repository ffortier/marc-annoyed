use crate::engine::Object3d;
use web_sys::{FontFace, HtmlImageElement};

pub struct AssetLoader {}

impl AssetLoader {
    pub async fn load_font(name: &str) -> FontFace {
        todo!()
    }

    pub async fn load_img(name: &str) -> HtmlImageElement {
        todo!()
    }

    pub async fn load_3d_object(name: &str) -> Object3d {
        todo!()
    }
}
