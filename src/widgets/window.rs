use gtk::{
    gio,
    glib::{self},   
    prelude::*,   
};
use crate::{
    application::Application,config,
    models::{ProvidersModel},


};

mod imp {
    use gtk::subclass::prelude::*;
    use std::{
        cell::{OnceCell},
    };
    use super::*;

    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(file = "../../ui/window.ui")]

    #[properties(wrapper_type = super::Window)]

  pub struct Window {
       #[property(get, set, construct_only)]
        pub model: OnceCell<ProvidersModel>,
       
        #[template_child]
        pub button_clicked5 : TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub button_clicked2 : TemplateChild<gtk::Button>,
  }



  #[glib::object_subclass]
   impl ObjectSubclass for Window {

         const NAME: &'static str = "Window";
         type Type = super::Window;
         type ParentType = gtk::ApplicationWindow;
         type Interfaces = (gio::Initable,);

            fn class_init(klass: &mut Self::Class) {
               
               klass.bind_template(); 
          
               klass.bind_template_instance_callbacks();



                klass.install_action("app.preferences0", None, |win, _, _| {

                       let button_clicked2 = &win.imp().button_clicked2;
                        button_clicked2.is_visible();
                        println!("app.preferences0!!!!");
                 });
                 klass.install_action("app.preferences1", None, |_win, _, _| {
                       println!("app.preferences1!!!!");
                 });

                 klass.install_action("app.preferences2", None, |win, _, _| {

                       println!("app.preferences2 Active!!!!");
                       
                       let button_clicked5 = &win.imp().button_clicked5;
						    button_clicked5.set_active(!button_clicked5.is_active());

                 });

                 klass.install_action("app.preferences5", None, |_win, _, _|{
                     println!("OK!!!");

                 });

            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {

              obj.init_template();

            }

   }


    #[glib::derived_properties]
    impl ObjectImpl for Window {
    
        fn constructed(&self) {
           self.parent_constructed();
             let win = self.obj();
             win.set_icon_name(Some(config::APP_ID));
        }

    }
       impl ButtonImpl for Window {}
        impl  WidgetImpl for Window {}
        impl  WindowImpl for Window {

           fn enable_debugging(&self, toggle: bool) -> bool {
            if config::PROFILE != "Devel" {
                //tracing::warn!("Inspector is disabled for non development builds");
                false
            } else {
                self.parent_enable_debugging(toggle)
            }
        }

        fn close_request(&self) -> glib::Propagation {
            /*if let Err(err) = self.obj().save_window_state() {
                tracing::warn!("Failed to save window state {:#?}", err);
            }
            */
            self.parent_close_request()
        }

        }
        impl  ApplicationWindowImpl for Window {}

        impl InitableImpl for Window {
             fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {

                let _win = self.obj();
                //let app = win.app();
                //win.action_set_enabled("win.search", !app.is_locked());

                //app.connect_is_locked_notify(clone!(@weak win => move |app| {



                //}));
               Ok(())
             }
        }

}


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::Initable, gio::ActionMap, gio::ActionGroup,gtk::Native,gtk::Root;
}

#[gtk::template_callbacks]
impl Window {
  pub fn new(model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
            .property("application", app)
            .property("model", model)
            .build(gio::Cancellable::NONE)
            .unwrap()
    }
       #[template_callback]
        fn on_search_btn_click(&self, _btn: &gtk::Button) {
           
            println!("on_search_btn_click!!");


        }

}
