# LBPH

A Local Binary Pattern (LBP) implementation exercise in Rust, built on
[kornia](https://kornia.org).

LBP codes have already been **precomputed** for the 5 images in `test_data/`
and stored in `test_data/lbp_expected.json`. Your call is to implement the two
`todo!()` functions in `src/lib.rs` and see if you get the same outputs —
`cargo test` will tell you.

## How LBP works

For every pixel of a **grayscale, 224x224** image (except the 1-pixel border):

1. Take the 3x3 neighborhood around the pixel.
2. Compare each of the 8 neighbors against the center pixel, going clockwise
   starting from the top-left: a neighbor `>=` center counts as `1`, else `0`.
3. Pack the 8 comparisons into a single `u8`, using weights `1, 2, 4, 8, 16,
   32, 64, 128` (top-left = `1` ... middle-left = `128`).

A 224x224 image yields `222 * 222 = 49284` LBP codes.

```text
thresholded   weights
1 1 0         1   2   4
0    1           8    16      ->  1+2+8+16+64 = 91
1 0 1         64  32  128
```

## The pipeline

`compute_lbp_numbers(path)` should:

1. Load the image from disk (8-bit RGB)
2. Convert it to grayscale
3. Resize it to 224x224 (bilinear)
4. Run `lbph_algorithm()` on the resized image

Useful kornia items: `kornia::io::functional::read_image_any_rgb8`,
`kornia::imgproc::color::gray_from_rgb_u8`,
`kornia::imgproc::resize::resize_fast_u8`, and `kornia::image::Image`.

## Repo layout

```text
src/
  lib.rs     the two todo!() functions to implement
  tests.rs   5 tests — one per image in test_data/
test_data/
  1.jpg ... 5.jpeg         the images to process
  lbp_expected.json        precomputed results, the ground truth
dataset/train/             source dataset (celebrity images) the
                           test images were taken from
```

## The tests

`test_data/lbp_expected.json` looks like:

```json
[{"imagePath": "1.jpg", "lbp": [251, 117, 230, ...]}, ...]
```

Each test loads its image, recomputes the LBP through your implementation,
and asserts the resulting 49284 values match the stored ones byte for byte:

```console
$ cargo test
running 5 tests
test tests::lbp_matches_1_jpg ... ok
test tests::lbp_matches_2_jpg ... ok
test tests::lbp_matches_3_jpg ... ok
test tests::lbp_matches_4_jpg ... ok
test tests::lbp_matches_5_jpeg ... ok

test result: ok. 5 passed; 0 failed
```

Until the `todo!()`s are implemented, all 5 tests fail with
`not yet implemented`.

## Notes

- Grayscale conversion and bilinear resizing must match the reference
  pipeline exactly, so use the kornia functions listed above.
- The fixture depends on `kornia 0.1.9`; a version bump can change resize
  results, in which case the JSON would need to be regenerated.
