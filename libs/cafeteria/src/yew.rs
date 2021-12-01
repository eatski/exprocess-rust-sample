use yew::{html, Callback, Html};

use crate::tree::{
    Directory as DirectoryCore, Gallery as GalleryCore, Picture as PictureCore, PictureTree,
};

pub type Directory = DirectoryCore<Html>;
pub type Pictures = PictureTree<Html>;
pub type Picture = PictureCore<Html>;
pub type GalleryModel = GalleryCore<Html>;

fn render(gallery: &GalleryModel, callback: Callback<Vec<String>>) -> Html {
    html! {
        render_dir(&gallery.dir,callback)
    }
}

#[cfg(test)]
mod test {
    use yew::{html, Callback};

    use crate::tree::picture;

    use super::{render, GalleryModel};

    #[test]
    fn it_works() {
        let model = GalleryModel {
            dir: [("hoge".to_owned(), picture(|| html! {}))].into(),
        };
        render(&model, Callback::noop());
    }
}

fn render_dir_with_name(name: &str, dir: &Directory, callback: Callback<Vec<String>>) -> Html {
    let name_string = name.to_string();
    let callback = callback.reform(move |mut vec: Vec<String>| {
        vec.push(name_string.clone());
        vec
    });
    html! {
        <>
            <p>{name}</p>
            {render_dir(dir,callback)}
        </>
    }
}

fn render_dir(dir: &Directory, callback: Callback<Vec<String>>) -> Html {
    let list = dir.iter().map(|(key, val)| {
        let content = match val {
            PictureTree::Picture(_) => {
                let key_cloned = key.clone();
                let callback = callback.reform(move |_| vec![key_cloned.clone()]);
                html! {<a onclick=callback>{key}</a>}
            }
            PictureTree::Dir(dir) => render_dir_with_name(key.as_str(), dir, callback.clone()),
        };
        html! {<li>{content}</li>}
    });
    html! {
        <ul>
            {for list}
        </ul>
    }
}
