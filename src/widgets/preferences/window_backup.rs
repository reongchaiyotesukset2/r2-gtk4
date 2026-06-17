use adw::prelude::*;
use gtk::{
    gio,
    glib::{self, clone},
    subclass::prelude::*,
};

use crate::{
    models::{ProvidersModel},
};

mod imp {
    use std::cell::Cell;

    use adw::subclass::prelude::*;

    use super::*;

   //  can't  gtk::CompositeTemplate
    #[derive(gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::PreferencesWindow)]
    //#[template(file = "../../../data/preferences.ui")]
    #[template(resource = "/org/example/myapp/preferences.ui")]

  pub struct PreferencesWindow {
      #[property(get, set, construct)]
      pub has_set_password: Cell<bool>,
  }

  #[glib::object_subclass]
   impl ObjectSubclass for PreferencesWindow {

         const NAME: &'static str = "PreferencesWindow";
         type Type = super::PreferencesWindow;
         type ParentType = gtk::ApplicationWindow;
         //type Interfaces = (gio::Initable,);

            fn class_init(klass: &mut Self::Class) {

               klass.bind_template();
               klass.bind_template_instance_callbacks();

            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {

              obj.init_template();

            }

   }


    #[glib::derived_properties]
    impl ObjectImpl for PreferencesWindow {

        fn constructed(&self) {

        }

    }
    impl WidgetImpl for PreferencesWindow {}
    impl WindowImpl for PreferencesWindow {}
}


glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::Initable, gio::ActionMap, gio::ActionGroup,gtk::Native,gtk::Root;
}

#[gtk::template_callbacks]
impl PreferencesWindow {
    pub fn new(model: &ProvidersModel)->Self{

           glib::Object::builder().property("model", model).build()

    }


}
