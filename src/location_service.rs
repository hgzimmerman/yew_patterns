use stdweb::web::Location;
use stdweb::web::window;

pub struct LocationService {}

impl LocationService {
    pub fn get_location(&self) -> Location {
        window().location().expect("Location should be present for browsers that support WASM")
    }

    pub fn set_location(&mut self, location: String) {
        let window = window();
        js! {
            @{window}.location.href = @{location};
        };
    }
}