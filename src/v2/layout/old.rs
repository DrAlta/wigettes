use std::collections::HashMap;
pub mod new;

#[derive(Debug, Clone)]
pub struct Rect<Spaxel> {
    pub width: Spaxel,
    pub height: Spaxel,
}

#[derive(Debug, Clone)]
pub enum Resolution {
    MischiefManaged,
    Unhandled,
}

#[derive(Debug, Clone)]
pub struct Layout<InterSpaxel, Spaxel, ChildId> {
    pub left: InterSpaxel,
    pub top: InterSpaxel,
    pub area: Rect<Spaxel>,
    pub children: HashMap<ChildId, Layout<InterSpaxel, Spaxel, ChildId>>,
}

pub trait FooFoo: Widget<i32, i32, usize, (), (), ()> + std::fmt::Debug {}
pub trait Widget<InterSpaxel, Spaxel, ChildId, World, Hairloom, Target> {
    fn get_child(
        &self,
        child_id: &ChildId,
        world: &World,
    ) -> Option<&dyn Widget<InterSpaxel, Spaxel, ChildId, World, Hairloom, Target>>;
    fn get_child_mut<'a, 'b>(
        &'a mut self,
        child_id: &'b ChildId,
    ) -> Box<dyn FnMut(&mut World) -> Option<&'a mut (dyn FooFoo + 'a)> + 'a>;
    //    fn get_child_mut<'a ,'b>(&'a mut self, child_id: &'b ChildId, world: &'a World) -> fn() -> Option<&'a mut dyn Widget<InterSpaxel, Spaxel, ChildId, World, Hairloom, Target>>;

    fn desired_portion(
        &self,
        offered: Rect<Spaxel>,
    ) -> Option<Layout<InterSpaxel, Spaxel, ChildId>>;
    fn draw_under_children(
        &self,
        left: InterSpaxel,
        top: InterSpaxel,
        area: Rect<Spaxel>,
        hairloom: Option<&Hairloom>,
        target: Target,
    ) -> Hairloom;
    fn draw_over_children(
        &self,
        left: InterSpaxel,
        top: InterSpaxel,
        area: Rect<Spaxel>,
        hairloom: Option<&Hairloom>,
        target: Target,
    );
    fn handle_click(
        &mut self,
        click_x: Spaxel,
        click_y: Spaxel,
        your_left: InterSpaxel,
        your_top: InterSpaxel,
        your_area: Rect<Spaxel>,
        hairloom: Option<&Hairloom>,
    ) -> Resolution;
}

#[derive(Debug)]
pub struct Foo {
    pub vec: Vec<Foo>,
}

impl FooFoo for Foo {}
impl Widget<i32, i32, usize, (), (), ()> for Foo {
    fn get_child(
        &self,
        _child_id: &usize,
        _world: &(),
    ) -> Option<&dyn Widget<i32, i32, usize, (), (), ()>> {
        todo!()
    }

    fn desired_portion(&self, _offered: Rect<i32>) -> Option<Layout<i32, i32, usize>> {
        todo!()
    }

    fn draw_under_children(
        &self,
        _left: i32,
        _top: i32,
        _area: Rect<i32>,
        _hairloom: Option<&()>,
        _target: (),
    ) -> () {
        todo!()
    }

    fn draw_over_children(
        &self,
        _left: i32,
        _top: i32,
        _area: Rect<i32>,
        _hairloom: Option<&()>,
        _target: (),
    ) {
        todo!()
    }

    fn handle_click(
        &mut self,
        _click_x: i32,
        _click_y: i32,
        _your_left: i32,
        _your_top: i32,
        _your_area: Rect<i32>,
        _hairloom: Option<&()>,
    ) -> Resolution {
        todo!()
    }

    fn get_child_mut<'a, 'b>(
        &'a mut self,
        child_id: &'b usize,
    ) -> Box<dyn FnMut(&mut ()) -> Option<&'a mut (dyn FooFoo + 'a)> + 'a> {
        let mut aa = self.vec.get_mut(*child_id).map(|f| f as &mut dyn FooFoo);
        Box::new(move |_| aa.take())
    }
}

#[test]
fn tst() {
    let mut w = ();
    let mut a = Foo {
        vec: vec![Foo {
            vec: vec![Foo { vec: Vec::new() }, Foo { vec: Vec::new() }],
        }],
    };
    println!("{a:?}");
    let mut c = a.get_child_mut(&0);
    println!("{:?}", c(&mut w));

    panic!();
}
