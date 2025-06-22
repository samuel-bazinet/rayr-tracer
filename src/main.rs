use simple_logger::SimpleLogger;

use ray_tracer::{file_utils::create_and_write_to_file, test_lib::create_test_image};

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Starting up");
    create_test_image();
}
