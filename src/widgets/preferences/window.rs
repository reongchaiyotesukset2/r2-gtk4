use adw::prelude::*;
use gtk::{
    gio,
    glib::{self, clone},
    subclass::prelude::*,
};


mod imp {
    use std::cell::Cell;

    use adw::subclass::prelude::*;

    use super::*;

    #[derive(gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::PreferencesWindow)]
    #[template(resource = "/org/example/myapp/preferences.ui")]
    pub struct PreferencesWindow {
        #[property(get, set, construct)]
        pub has_set_password: Cell<bool>,
        pub actions: gio::SimpleActionGroup,

    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = adw::PreferencesDialog;

        fn new() -> Self {
            let actions = gio::SimpleActionGroup::new();

            Self {
                has_set_password: Cell::default(), // Synced from the application
                actions,
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
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup_actions();
            obj.setup_widget();
        }
    }
    impl WidgetImpl for PreferencesWindow {}
    impl AdwDialogImpl for PreferencesWindow {}
    impl PreferencesDialogImpl for PreferencesWindow {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
    @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl PreferencesWindow {
    fn setup_widget(&self) {
        let imp = self.imp();

    }

    fn setup_actions(&self) {
        let imp = self.imp();

    }
}

impl Default for PreferencesWindow {
    fn default() -> Self {
        glib::Object::new()
    }
}
