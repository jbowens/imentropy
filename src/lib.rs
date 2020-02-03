extern crate image;

use std::collections::HashMap;

/// entropy calculates the entropy of an image's intensities.
/// The provided image is converted to grayscale, a histogram
/// of the grayscale values is constructed, and the
/// formula -sum(p*log2(p)) is calculated.
pub fn entropy(im: image::DynamicImage) -> f64 {
    let gray = image::imageops::colorops::grayscale(&im);

    // Calculate the histogram of grayscale intensities.
    let mut hist = HashMap::<u8, u64>::new();
    for pixel in gray.pixels() {
        let x : u8 = pixel[0];
        let count = hist.entry(x).or_default();
        *count += 1;
    }

    let pixel_count = (gray.width() * gray.height()) as f64;
    let mut entropy_sum = 0f64;
    for count in hist.values() {
        let p = *count as f64 / pixel_count;
        entropy_sum += p * p.log(2.0);
    }
    -entropy_sum
}

#[cfg(test)]
mod tests {
    use super::entropy;

    #[test]
    fn entropy_test_files() {
        let test_cases = vec![
            "test/bakers-hotline.png",
            "test/biscotti-large.jpg",
        ];
        for filename in test_cases {
            let img = image::open(filename).unwrap();
            let e = entropy(img);
            println!("entropy({}) = {}", filename, e);
            // TODO: add asserts
        }
    }
}
