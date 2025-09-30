use gtk::{
    gdk_pixbuf, gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};
use diesel::prelude::*;
use crate::{
    models::{database},
    schema::providers,
};

mod imp {
    use std::cell::{Cell, RefCell};
    use super::*;

    #[derive(glib::Properties)]
    #[properties(wrapper_type = super::Provider)]
    pub struct Provider {
        #[property(get, set, construct_only)]
        pub id: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Provider {
        const NAME: &'static str = "Provider";
        type Type = super::Provider;
        fn new() -> Self {
            Self {
                id: Cell::default(),
            }
        }
    }

    impl ObjectImpl for Provider {

    }

    impl Provider {

    }
}

glib::wrapper! {
    pub struct Provider(ObjectSubclass<imp::Provider>);
}

impl Provider {
    pub fn load()-> Result<(), Box<dyn std::error::Error>> {
        println!("load on provider on provider.rs");

        let db = database::connection();
        let mut conn = db.get()?;
        diesel::delete(providers::table.filter(providers::columns::id.eq(2)))
        .execute(&mut conn)?;
        Ok(())
    }
}
