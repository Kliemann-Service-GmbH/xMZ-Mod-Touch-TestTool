extern crate glib_sys;
extern crate glib;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate libc;

use errors::*;
use gdk::enums::key;
use gtk;
use gtk::prelude::*;
use self::glib_sys::gpointer;
use self::glib::translate::ToGlibPtr;
use std::thread;
use std::time::Duration;

mod index;
mod static_resource;    // Zur Einbindung der .gresource Datei


// Basic Setup des Fensters
fn window_main_setup(window: &gtk::Window) -> Result<()> {
    let window_title = format!("{} {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    window.set_default_size(1024, 600);
    // window.set_border_width(10);

    let display = window.get_display().unwrap();
    let screen = display.get_screen(0);
    screen.set_resolution(130.0);

    #[cfg(not(feature = "development"))]
    window.fullscreen();

    Ok(())
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
        env!("CARGO_PKG_NAME")))
    });

    static_resource::init();    // Inititialisieren der .gresource

    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let builder = gtk::Builder::new_from_resource("/com/gaswarnanlagen/xmz-mod-touch-test-tool/GUI/main.ui");

    let window_main: gtk::Window = builder.get_object("window_main").unwrap();

    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_main_setup(&window_main);

    { // Hide info_bar
            let info_bar = info_bar.clone();
            info_bar.connect_response(move |info_bar, _| info_bar.hide());
    }


    window_main.show_all();
    info_bar.hide();

    index::run();


    // // 1Sek Thread
    // gtk::idle_add(move || {
    //     thread::sleep(Duration::from_millis(1000));
    //
    //     glib::Continue(true)
    // });

    // Beende Programm wenn das Fenster geschlossen wurde
    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    gtk::main();
}
