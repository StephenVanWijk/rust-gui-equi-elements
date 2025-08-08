mod initialization_gui_elements;

use eframe::Error;
use initialization_gui_elements::run_app;

fn main() -> eframe::Result<(), Error> {
    let result: Result<(), Error> =  run_app();
    result.map_err(|e| Error::from(e))
}

