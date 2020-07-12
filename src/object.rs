use crate::ray::Ray;

pub struct Object
{
    field: Box<dyn Fn(Ray) -> Vec<f64>>,
}

impl Object
where
{
    fn new(field: Box<dyn Fn(Ray) -> Vec<f64>>) -> Self {
        Self { field }
    }

    // Combine the fields of two objects
    fn combine( field1: Box<dyn Fn(Ray) -> Vec<f64>>
              , field2: Box<dyn Fn(Ray) -> Vec<f64>>)
              -> Box<dyn Fn(Ray) -> Vec<f64> + 'static>
    {
        Box::new(move |ray: Ray| {
            let mut res = vec![];
            res.append(&mut field1(ray));
            res.append(&mut field2(ray));
            return res;
        })
    }

    // Combine two objects
    fn compose(self, other: Object) -> Object {
        Object::new( Self::combine(Box::new(self.field), Box::new(other.field)) )
    }
}
