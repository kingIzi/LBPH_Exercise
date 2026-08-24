use kornia::{
    image::{Image, ImageSize},
    imgproc::{self, resize::resize_fast_u8},
    io::functional::read_image_any_rgb8,
    tensor::CpuAllocator,
};
use ndarray::Array1;

/// Computes the Local Binary Pattern codes for a 224x224 grayscale image.
///
/// For every pixel except the 1-pixel border, compare the center pixel
/// against its 8 neighbors (clockwise from top-left) and pack the
/// neighbor >= center bits into a single u8 (0..255).
pub fn lbph_algorithm(gray_resized: &Image<u8, 1, CpuAllocator>) -> Vec<u8> {
    let mut output = Vec::with_capacity(222 * 222);
    for y in 1..gray_resized.height() - 1 {
        for x in 1..gray_resized.width() - 1 {
            // 3. Get the center pixel
            // Kornia's .get() takes [y, x, channel]
            let center = *gray_resized.get([y, x, 0]).unwrap_or(&0);

            // 4. Get neighbors in clockwise order starting from Top-Left
            let top_left = *gray_resized.get([y - 1, x - 1, 0]).unwrap_or(&0) >= center;
            let top_middle = *gray_resized.get([y - 1, x, 0]).unwrap_or(&0) >= center;
            let top_right = *gray_resized.get([y - 1, x + 1, 0]).unwrap_or(&0) >= center;
            let middle_right = *gray_resized.get([y, x + 1, 0]).unwrap_or(&0) >= center;
            let bottom_right = *gray_resized.get([y + 1, x + 1, 0]).unwrap_or(&0) >= center;
            let bottom_middle = *gray_resized.get([y + 1, x, 0]).unwrap_or(&0) >= center;
            let bottom_left = *gray_resized.get([y + 1, x - 1, 0]).unwrap_or(&0) >= center;
            let middle_left = *gray_resized.get([y, x - 1, 0]).unwrap_or(&0) >= center;

            // 5. Calculate the LBP integer value
            let mut lbp_val: u8 = 0;
            if top_left {
                lbp_val += 1;
            }
            if top_middle {
                lbp_val += 2;
            }
            if top_right {
                lbp_val += 4;
            }
            if middle_right {
                lbp_val += 8;
            }
            if bottom_right {
                lbp_val += 16;
            }
            if bottom_middle {
                lbp_val += 32;
            }
            if bottom_left {
                lbp_val += 64;
            }
            if middle_left {
                lbp_val += 128;
            }
            output.push(lbp_val);
        }
    }
    output
}

/// Loads an image from disk and returns its 49284 LBP codes (222 * 222).
pub fn compute_lbp_numbers(img_path: &str) -> Vec<u8> {
    let image_rgb = read_image_any_rgb8(img_path).expect("Faied to find image");

    let mut gray = Image::<u8, 1, _>::from_size_val(image_rgb.size(), 0, CpuAllocator)
        .expect("Failed to get size");
    imgproc::color::gray_from_rgb_u8(&image_rgb, &mut gray)
        .expect("Faled to convert image to gray scale");
    let new_size = ImageSize {
        width: 224,
        height: 224,
    };
    let mut gray_resized =
        Image::<u8, 1, _>::from_size_val(new_size, 0, CpuAllocator).expect("Failed to get size");
    resize_fast_u8(
        &gray,
        &mut gray_resized,
        imgproc::interpolation::InterpolationMode::Bilinear,
    )
    .expect("Failed to resize grayscale image");

    lbph_algorithm(&gray_resized)
}

pub fn compute_histogram(lbp_numbers: &Vec<u8>) -> Array1<f32> {
    let mut histogram = vec![0u32; 256];
    lbp_numbers.iter().for_each(|value| {
        histogram[*value as usize] += 1;
    });
    let total = lbp_numbers.len() as f32;
    Array1::from(
        histogram
            .into_iter()
            .map(|count| count as f32 / total)
            .collect::<Vec<_>>(),
    )
}
