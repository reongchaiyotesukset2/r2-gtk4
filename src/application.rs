use adw::prelude::*;
use gtk::{
  gio,
  glib::{self},
  subclass::prelude::*,
  prelude::*,
};
use crate::{
  widgets::{Window,PreferencesWindow},
  models::{ProvidersModel},
};
use crate::config;
use gtk::subclass::prelude::DerivedObjectProperties;
use std::cell::{Cell,RefCell};
use gettextrs::gettext;

mod imp {
  use adw::subclass::prelude::*;


  use super::*;
  #[derive(Default,glib::Properties)]
  #[properties(wrapper_type = super::Application)]

  pub struct Application {
    pub window: RefCell<Option<glib::WeakRef<Window>>>,
    pub model: ProvidersModel,
    #[property(get, set, construct)]
    pub is_locked: Cell<bool>,

  }


  #[glib::object_subclass]
  impl ObjectSubclass for Application {

    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = adw::Application;
    type Interfaces = ();
  }
  #[glib::derived_properties]
  impl ObjectImpl for Application { }

  impl ApplicationImpl for Application {

    fn startup(&self) {

      println!("startup");

      self.parent_startup();

      let app = self.obj();



      let quit_action = gio::ActionEntry::builder("quit")
      .activate(|app: &Self::Type, _, _| {
        app.quit()
      })
      .build();
      let preferences_action = gio::ActionEntry::builder("preferences")
      .activate(|app: &Self::Type, _, _| {
        let window = app.active_window();
        let preferences = PreferencesWindow::default();
        preferences.present(Some(&window));

      })
      .build();

      // About
      let about_action = gio::ActionEntry::builder("about")
      .activate(|app: &Self::Type, _, _| {
        let window = app.active_window();
        adw::AboutDialog::builder()
        .application_name(gettext("Authenticator"))
        .version(config::VERSION)
        .comments(gettext("Generate two-factor codes"))
        .website("https://gitlab.gnome.org/World/Authenticator")
        .developers(vec![
          "Bilal Elmoussaoui",
          "Maximiliano Sandoval",
          "Christopher Davis",
          "Julia Johannesen",
        ])
        .artists(vec!["Alexandros Felekidis", "Tobias Bernard"])
        .translator_credits(gettext("translator-credits"))
        .application_icon(config::APP_ID)
        .license_type(gtk::License::Gpl30)
        .build()
        .present(Some(&window));
      })
      .build();


      app.add_action_entries([
        quit_action,
        preferences_action,
        about_action,
      ]);


    }

    fn activate(&self) {
      println!("activate");
      let app = self.obj();
      let window = Window::new(&self.model, &app);
      window.present();
      self.window.replace(Some(window.downgrade()));

      app.set_accels_for_action("app.preferences", &["<primary>comma"]);
    }
    fn open(&self, _files: &[gio::File], _hint: &str)
    {
      //don't active
      self.activate();
    }


  }
  impl GtkApplicationImpl for Application {}
  impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
  pub struct Application(ObjectSubclass<imp::Application>)
  @extends gio::Application, gtk::Application, adw::Application,
  @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {

  pub fn run() -> glib::ExitCode  {
    println!("run!!!!!!!!!!!!");

    //tracing::info!("Authenticator ({})", config::APP_ID);
    //tracing::info!("Version: {} ({})", config::VERSION, config::PROFILE);
    //let test = self.obj();

    let app = glib::Object::builder::<Application>()
    .property("application-id", config::APP_ID)
    .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
    .build();
    app.imp().model.load();
    app.run()

    // Self::test2();

  }
  pub fn test2(){
    println!("test2!!!!!");

  }
  pub fn test1(&self){
    println!("test1!!!!!!");
    //self.imp().defbool;
    //let applicatiion_test = self.imp().activate();
  }
  pub fn active_window(&self) -> Window {
    self.imp()
    .window
    .borrow()
    .as_ref()
    .unwrap()
    .upgrade()
    .unwrap()
  }


}
