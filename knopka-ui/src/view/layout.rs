use crate::view::View;

pub struct Layout {
    views: Vec<Box<dyn View>>,
}

impl Layout {
    pub fn column() -> Self {
        unimplemented!()
    }

    pub fn row() -> Self {
        unimplemented!()
    }

    pub fn views(mut self, views: impl IntoViews) -> Self {
        self.views = views.into_views();
        self
    }
}

impl View for Layout {}

pub trait IntoViews {
    fn into_views(self) -> Vec<Box<dyn View>>;
}

macro_rules! impl_into_views_for_tuple {
    ($($x:ident),*) => {
        impl<$($x: View),*,> IntoViews for ($($x),*,) {
            #[allow(non_snake_case)]
            fn into_views(self) -> Vec<Box<dyn View>> {
                let ($($x),*,) = self;

                vec![
                    $(Box::new($x)),*,
                ]
            }
        }
    };
}

impl_into_views_for_tuple!(A);
impl_into_views_for_tuple!(A, B);
impl_into_views_for_tuple!(A, B, C);
impl_into_views_for_tuple!(A, B, C, D);
impl_into_views_for_tuple!(A, B, C, D, E);
