use gtk::{

    glib::{self},
    subclass::prelude::*,
};

mod imp {

    use super::*;

     #[derive(Default)]
    pub struct Provider {
       
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Provider {
        const NAME: &'static str = "Provider";
        type Type = super::Provider;

        
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
    
}
