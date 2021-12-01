use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::tree::{dir, picture};

    #[test]
    fn it_works() {
        let tree = dir([
            ("hoge",picture(|| 2)),
            ("fuga",dir([
                ("piyo",picture(|| 3))
            ]))
        ]);
        let result = tree
            .get(["fuga","piyo"].into_iter().map(String::from))
            .map(|picture| (picture.func)());
        assert_eq!(result, Some(3));
    }
}
pub type Directory<N> = HashMap<String,PictureTree<N>>;
pub enum PictureTree<N> {
    Picture(Picture<N>),Dir(HashMap<String,PictureTree<N>>)
}

pub struct Gallery<N> {
    pub dir: Directory<N>
}

impl <N> PictureTree<N> {
    pub fn get<I: Iterator<Item = String>>(&self,iter: I) -> Option<&Picture<N>> {
        let tree = iter.fold(Some(self), |prev,cur| {
            match prev {
                Some(PictureTree::Dir(map)) => map.get(&cur),
                _ => None,
            }
        });
        match tree {
            Some(PictureTree::Picture(picture)) => Some(picture),
            _ => None,
        }
    }
}

pub fn picture<N,F: Fn() -> N + 'static>(f: F) -> PictureTree<N>{
    PictureTree::Picture(Picture::new(f))
}

pub fn dir<'a,N,In:Into<Vec<(&'a str,PictureTree<N>)>>>(val: In) -> PictureTree<N>{
    let vec: Vec<(&'a str,PictureTree<N>)> = val.into();
    PictureTree::Dir(vec.into_iter().map(|(name,picture)| (name.to_string(),picture)).collect())
}

pub struct Picture<N> {
    func: Box<dyn Fn() -> N>
}

impl <N> Picture<N> {
    pub fn new<F: Fn() -> N + 'static>(f: F) -> Self {
        Picture { func: Box::new(f) }
    }
    pub fn get(&self) -> N {
        (self.func)()
    }
}