use honggfuzz::fuzz;
use lsd::app::validate_time_format;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(arg) = std::str::from_utf8(data) {
                let _ = validate_time_format(arg);

            }
        });
    }
}