
pub struct WriteableBox {
    start_x: u8,
    start_y: u8,

    size_x: u8,
    size_y: u8,

}

pub struct Division<T1: Boxable, T2: Boxable> {
    box1: Box<T1>,
    box2: Box<T2>,

    percentage: u8

}

pub trait Boxable {
    fn merge<'a>(&'a self, b: &'a dyn Boxable, percentage: u8) -> &Division<&dyn Boxable, &dyn Boxable>;
}

impl Boxable for &WriteableBox {
    fn merge<'a>(&'a self, b: &'a dyn Boxable, percentage: u8) -> &Division<&dyn Boxable, &dyn Boxable>{
        return &(Division {
            box1: Box::new(self),
            box2: Box::new(b),

            percentage: percentage
        });

    }
}
//impl Boxable for &Division<'_, &Box, &Box> {}
/*impl Boxable for &Division<&Box, &Division<&Boxable, &Boxable>> {}
impl Boxable for Division<Division<dyn Boxable, dyn Boxable>, Box> {}
impl Boxable for Division<Division<dyn Boxable, dyn Boxable>, Division<dyn Boxable, dyn Boxable>> {}*/
impl Boxable for &Division<&dyn Boxable, &dyn Boxable> {
    fn merge<'a>(&'a self, b: &'a dyn Boxable, percentage: u8) -> &Division<&dyn Boxable, &dyn Boxable> {
        return &(Division {
            box1: Box::new(self),
            box2: Box::new(b),

            percentage: percentage
        });

    }

}
impl Boxable for &dyn Boxable{
    fn merge<'a>(&'a self, b: &'a dyn Boxable, percentage: u8) -> &Division<&dyn Boxable, &dyn Boxable> {
        return &(Division {
            box1: Box::new(self),
            box2: Box::new(b),

            percentage: percentage
        });

    }
}



