use gst::glib;
use gst::glib::prelude::*;
use gst::glib::subclass::prelude::*;

pub mod imp {
    use super::*;

    #[derive(Default)]
    pub struct Foo {}

    #[glib::object_subclass]
    impl ObjectSubclass for Foo {
        const NAME: &'static str = "MyFoo";
        type Type = super::Foo;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for Foo {}
}

pub trait FooExt: 'static {
    fn test(&self);
}

impl<O: IsA<Foo>> FooExt for O {
    fn test(&self) {
        let _self = imp::Foo::from_instance(self.as_ref().downcast_ref::<Foo>().unwrap());
        unimplemented!();
    }
}

glib::wrapper! {
    pub struct Foo(ObjectSubclass<imp::Foo>);
}
