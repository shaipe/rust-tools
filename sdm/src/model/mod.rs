
use crate::tar::Model;

pub struct Test {

    pub id: String,

    pub ss: String,

    pub parent: Box<Test>
}

// impl Model for Test {
//     fn parse(&self) -> Option<Box<impl Model>> {

//     }
// }