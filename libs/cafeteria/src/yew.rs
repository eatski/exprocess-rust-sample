use std::{marker::PhantomData};

use yew::{html, Callback, Html, Component, ComponentLink};

use crate::tree::{
    Directory as DirectoryCore, Gallery as GalleryCore, Picture as PictureCore, PictureTree,
};

pub use crate::tree::{dir,picture};

pub type Directory = DirectoryCore<Html>;
pub type Pictures = PictureTree<Html>;
pub type Picture = PictureCore<Html>;
pub type GalleryModel = GalleryCore<Html>;

pub trait GalleryConfig {
    fn model() -> GalleryModel;
}

pub struct Gallery<C: GalleryConfig + 'static> {
    __marker: PhantomData<C>,
    current: PicturePath,
    model: GalleryModel,
    link: ComponentLink<Self>
}

impl <C: GalleryConfig + 'static>Component for Gallery<C> {
    type Message = PicturePath;

    type Properties = ();

    fn create(_props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            __marker: PhantomData,
            current: Vec::new(),
            model: C::model(),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        self.current = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        let current = self.model.get(self.current.iter().cloned());
        html! {
            <div class="cafeteria-root">
                <section>
                    <h2>{self.current.join("/")}</h2>
                    {self.render_tree(self.link.callback(|v| v))}
                </section>
                {current.map(|current| { html! { <section>{current}</section> } }).unwrap_or_default()}
            </div>
        }
    }
}

impl <C: GalleryConfig + 'static>Gallery<C> {
    fn render_tree(&self,callback: Callback<PicturePath>) -> Html {
        self.tree_dir(&self.model.dir,callback,Vec::new())
    }
    fn tree_dir_with_name(&self,name: &str, dir: &Directory, callback: Callback<PicturePath>,path: PicturePath) -> Html {
        html! {
            <>
                <p>{name}</p>
                {self.tree_dir(dir,callback,path)}
            </>
        }
    }
    fn tree_dir(&self,dir: &Directory, callback: Callback<PicturePath>,path: PicturePath) -> Html {
        let list = dir.iter().map(move |(key, val)| {
            let mut path = path.clone();
            path.push(key.clone());
            let content = match val {
                PictureTree::Picture(_) => {
                    let path_cloned = path.clone();
                    let callback = callback.reform(move |_| path_cloned.clone());
                    if path.eq(&self.current) { html! {<strong>{key}</strong>} } else { html! {<a onclick=callback>{key}</a>} }
                }
                PictureTree::Dir(dir) => self.tree_dir_with_name(key.as_str(), dir, callback.clone(),path.clone()),
            };
            html! {<li>{content}</li>}
        });
        html! {
            <ul>
                {for list}
            </ul>
        }
    }
    
}

pub type PicturePath = Vec<String>;