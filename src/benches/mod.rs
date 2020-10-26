extern crate test;

use crate::extractor;
use self::test::Bencher;

#[bench]
fn gen_filename(b: &mut Bencher) {
    b.iter(|| extractor::generate_output_file("image/demo.jpeg"))
}

#[bench]
fn write_date(b: &mut Bencher) {
    b.iter(|| extractor::write_to_file(
        "This is for bench test of project imagextractor".parse().unwrap(), "/tmp/imagextractor_test.json".to_string()
    ))
}

#[bench]
fn extrac_image_info(b: &mut Bencher) {
    b.iter(|| {
        let result = extractor::extract_img_info("images/JAM19896.jpg");
        assert!(result.is_ok())
    })
}
