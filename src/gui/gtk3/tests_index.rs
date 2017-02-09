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



// Basic Setup des Fensters
fn window_main_setup(window: &gtk::Window) -> Result<()> {
    let window_title = format!("{} {}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"));
    window.set_title(&window_title);
    // window.set_default_size(1024, 600);
    // window.set_border_width(10);

    // let display = window.get_display().unwrap();
    // let screen = display.get_screen(0);
    // screen.set_resolution(150.0);

    #[cfg(not(feature = "development"))]
    window.fullscreen();
    // // window.maximize();

    Ok(())
}

pub fn launch() {
    gtk::init().unwrap_or_else(|_| {
        panic!(format!("{}: GTK konnte nicht initalisiert werden.",
        env!("CARGO_PKG_NAME")))
    });

    ::gui::gtk3::static_resource::init();    // Inititialisieren der .gresource

    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let builder = gtk::Builder::new_from_resource("/com/gaswarnanlagen/xmz-mod-touch-test-tool/GUI/main.ui");

    let window_main: gtk::Window = builder.get_object("window_main").unwrap();
    let button_test_relais_all: gtk::ToggleButton = builder.get_object("button_test_relais_all").unwrap();
    let button_test_leds_all: gtk::ToggleButton = builder.get_object("button_test_leds_all").unwrap();
    let button_test_relais_random: gtk::ToggleButton = builder.get_object("button_test_relais_random").unwrap();
    let button_test_leds_random: gtk::ToggleButton = builder.get_object("button_test_leds_random").unwrap();
    let button_test_relais_one_after_one: gtk::ToggleButton = builder.get_object("button_test_relais_one_after_one").unwrap();
    let button_test_leds_one_after_one: gtk::ToggleButton = builder.get_object("button_test_leds_one_after_one").unwrap();

    let button_relais1: gtk::ToggleButton = builder.get_object("button_relais1").unwrap();
    let button_relais2: gtk::ToggleButton = builder.get_object("button_relais2").unwrap();
    let button_relais3: gtk::ToggleButton = builder.get_object("button_relais3").unwrap();
    let button_relais4: gtk::ToggleButton = builder.get_object("button_relais4").unwrap();
    let button_relais5: gtk::ToggleButton = builder.get_object("button_relais5").unwrap();
    let button_relais6: gtk::ToggleButton = builder.get_object("button_relais6").unwrap();
    let button_relais7: gtk::ToggleButton = builder.get_object("button_relais7").unwrap();
    let button_relais8: gtk::ToggleButton = builder.get_object("button_relais8").unwrap();
    let button_relais9: gtk::ToggleButton = builder.get_object("button_relais9").unwrap();

    let button_led1: gtk::ToggleButton = builder.get_object("button_led1").unwrap();
    let button_led2: gtk::ToggleButton = builder.get_object("button_led2").unwrap();
    let button_led3: gtk::ToggleButton = builder.get_object("button_led3").unwrap();
    let button_led4: gtk::ToggleButton = builder.get_object("button_led4").unwrap();
    let button_led5: gtk::ToggleButton = builder.get_object("button_led5").unwrap();
    let button_led6: gtk::ToggleButton = builder.get_object("button_led6").unwrap();
    let button_led7: gtk::ToggleButton = builder.get_object("button_led7").unwrap();
    let button_led8: gtk::ToggleButton = builder.get_object("button_led8").unwrap();
    let button_led9: gtk::ToggleButton = builder.get_object("button_led9").unwrap();
    let button_led10: gtk::ToggleButton = builder.get_object("button_led10").unwrap();
    let button_led11: gtk::ToggleButton = builder.get_object("button_led11").unwrap();
    let button_led12: gtk::ToggleButton = builder.get_object("button_led12").unwrap();
    let button_led13: gtk::ToggleButton = builder.get_object("button_led13").unwrap();
    let button_led14: gtk::ToggleButton = builder.get_object("button_led14").unwrap();
    let button_led15: gtk::ToggleButton = builder.get_object("button_led15").unwrap();
    let button_led16: gtk::ToggleButton = builder.get_object("button_led16").unwrap();
    let button_led17: gtk::ToggleButton = builder.get_object("button_led17").unwrap();
    let button_led18: gtk::ToggleButton = builder.get_object("button_led18").unwrap();
    let button_led19: gtk::ToggleButton = builder.get_object("button_led19").unwrap();
    let button_led20: gtk::ToggleButton = builder.get_object("button_led20").unwrap();

    let info_bar: gtk::InfoBar = builder.get_object("info_bar").unwrap();

    // Rufe Funktion für die Basis Fenster Konfiguration auf
    window_main_setup(&window_main);

    { // Hide info_bar
            let info_bar = info_bar.clone();
            info_bar.connect_response(move |info_bar, _| info_bar.hide());
    }

    button_test_relais_all.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::all(button);
    }));

    button_test_leds_all.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::all(button);
    }));

    button_test_relais_random.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::random(button);
    }));

    button_test_leds_random.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::random(button);
    }));

    button_test_relais_one_after_one.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::one_after_one(button);
    }));

    button_test_leds_one_after_one.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::one_after_one(button);
    }));


    button_relais1.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 1);
    }));
    button_relais2.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 2);
    }));
    button_relais3.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 3);
    }));
    button_relais4.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 4);
    }));
    button_relais5.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 5);
    }));
    button_relais6.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 6);
    }));
    button_relais7.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 7);
    }));
    button_relais8.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 8);
    }));
    button_relais9.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::relais_controller::set(button, 9);
    }));


    button_led1.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 1);
    }));
    button_led2.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 2);
    }));
    button_led3.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 3);
    }));
    button_led4.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 4);
    }));
    button_led5.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 5);
    }));
    button_led6.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 6);
    }));
    button_led7.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 7);
    }));
    button_led8.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 8);
    }));
    button_led9.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 9);
    }));
    button_led10.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 10);
    }));
    button_led11.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 11);
    }));
    button_led12.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 12);
    }));
    button_led13.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 12);
    }));
    button_led14.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 13);
    }));
    button_led15.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 14);
    }));
    button_led16.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 15);
    }));
    button_led17.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 16);
    }));
    button_led18.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 17);
    }));
    button_led19.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 18);
    }));
    button_led20.connect_clicked(clone!(builder => move |button| {
        ::gui::gtk3::leds_controller::set(button, 19);
    }));


    window_main.show_all();
    info_bar.hide();


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

    #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    gtk::main();
}
