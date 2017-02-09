mod leds_controller;
mod leds_index;
mod relais_controller;
mod relais_index;
mod static_resource;    // Zur Einbindung der .gresource Datei
mod tests_index;


pub fn launch() {
    tests_index::launch();
}
