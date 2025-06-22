use simple_logger::SimpleLogger;

use ray_tracer::test_lib::create_test_image;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Starting up");
    create_test_image();
}
