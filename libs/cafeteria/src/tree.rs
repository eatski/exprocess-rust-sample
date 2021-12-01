use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use crate::tree::{dir, picture};
    use crate::tree::Gallery;

    #[test]
    fn it_works() {
        let tree = Gallery::new(
        [
                ("hoge",picture(|| 2)),
                ("fuga",dir([
                    ("piyo",picture(|| 3))
                ]))
            ]
        );
        let result = tree
            .get(["fuga","piyo"].into_iter().map(String::from));
        assert_eq!(result, Some(3));
    }
}
pub type Directory<N> = HashMap<String,PictureTree<N>>;

pub struct Gallery<N> {
    pub dir: Directory<N>
}

impl <N>Gallery<N> {
    pub fn get<I: Iterator<Item = String>>(&self,mut iter: I) -> Option<N> {
        let illegal_access = "Illegal Access";
        let first = iter.next();
        first
            .map(|first| self.dir.get(&first).expect(illegal_access))
            .map(|tree| iter.fold(tree, |prev,cur|{
                match prev {
                    PictureTree::Dir(map) => map.get(&cur).expect(illegal_access),
                    _ => panic!("{}",illegal_access),
                }
            }))
            .map(|last| {
                match last {
                    PictureTree::Picture(picture) => picture.get(),
                    PictureTree::Dir(_) => panic!("{}",illegal_access),
                }
            })
    }
    pub fn new<'a,In:Into<Vec<(&'a str,PictureTree<N>)>>>(val: In) -> Self {
        let vec: Vec<(&'a str,PictureTree<N>)> = val.into();
        Self {
            dir: vec.into_iter().map(|(name,picture)| (name.to_string(),picture)).collect()
        }
    }
}
pub enum PictureTree<N> {
    Picture(Picture<N>),Dir(HashMap<String,PictureTree<N>>)
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