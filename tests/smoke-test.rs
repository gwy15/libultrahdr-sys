#[test]
fn main() {
    let encoder = unsafe { libultrahdr_sys::uhdr_create_encoder() };

    unsafe { libultrahdr_sys::uhdr_encode(encoder) };

    unsafe {
        libultrahdr_sys::uhdr_release_encoder(encoder);
    }
}
