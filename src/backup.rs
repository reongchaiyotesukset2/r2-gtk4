//use adw::prelude::*;
//use anyhow::Result;
//use gettextrs::gettext;
use gtk::{
    gio,
    glib::{self, clone},
    subclass::prelude::*,
};

//use super::{camera_page::CameraPage, password_page::PasswordPage};
use crate::{
  
    models::{ProvidersModel},
};
 use gtk::prelude::ObjectExt;
mod imp {
    use std::{
        cell::{Cell, OnceCell, RefCell},
        collections::HashMap,
    };


    use super::*;

    #[derive(gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::PreferencesWindow)]
   	#[template(file = "../../../ui/preferences.ui")]
	
    pub struct PreferencesWindow {
		#[property(get, set, construct_only)]
        pub model: OnceCell<ProvidersModel>,

        #[property(get, set, construct)]
        pub has_set_password: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = gtk::ApplicationWindow;

        fn new() -> Self {
            let actions = gio::SimpleActionGroup::new();

            Self {
				model: OnceCell::default(),
                has_set_password: Cell::default(),
                
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PreferencesWindow {
		/*
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("restore-completed").action().build()]);
            SIGNALS.as_ref()
        }
		*/

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup_actions();
            obj.setup_widget();
        }
    }
    impl WidgetImpl for PreferencesWindow {}
    //impl AdwDialogImpl for PreferencesWindow {}
   // impl PreferencesDialogImpl for PreferencesWindow {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends gtk::Widget;
}

impl PreferencesWindow {
    pub fn new(model: &ProvidersModel)->Self{
	   
           glib::Object::builder().property("model", model).build()
	  
    }
    
    pub fn connect_restore_completed<F>(&self, callback: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_local(
            "restore-completed",
            false,
            clone!(@weak self as win => @default-return None, move |_| {
                callback(&win);
                None
            }),
        )
    }
   
    pub fn setup_widget(&self) {
    
		println!("Setup_widget!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!1");
    }

    fn setup_actions(&self) {
        let imp = self.imp();

    }
}
