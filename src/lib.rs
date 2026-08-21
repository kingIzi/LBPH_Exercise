use kornia::image::Image;
use kornia::tensor::CpuAllocator;

#[cfg(test)]
mod tests;

/// Computes the Local Binary Pattern codes for a 224x224 grayscale image.
///
/// For every pixel except the 1-pixel border, compare the center pixel
/// against its 8 neighbors (clockwise from top-left) and pack the
/// neighbor >= center bits into a single u8 (0..255).
pub fn lbph_algorithm(_gray_resized: &Image<u8, 1, CpuAllocator>) -> Vec<u8> {
    // 1. Iterate over every pixel except the 1-pixel border
    //    (y in 1..height-1, x in 1..width-1)
    // 2. Read the center pixel
    // 3. Read the 8 neighbors and compare each against the center
    // 4. Pack the comparisons into a single u8 (bit weights 1, 2, 4, ..., 128)
    // 5. Push the value into the output vector
    todo!()
}

/// Loads an image from disk and returns its 49284 LBP codes (222 * 222).
pub fn compute_lbp_numbers(_img_path: &str) -> Vec<u8> {
    // 1. Load the image from disk (RGB, 8-bit)
    // 2. Convert the image to grayscale
    // 3. Resize the grayscale image to 224x224
    // 4. Run lbph_algorithm on the resized image
    todo!()
}
