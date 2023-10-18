use std::vec;

use icu::segmenter::WordSegmenter;

// Import `Segmenter` trait.
use crate::segmenter::Segmenter;

extern crate alloc; // required as my-data-mod is written for #[no_std]
use icu_provider_blob::BlobDataProvider;
//TIP: Some segmentation Libraries need to initialize a instance of the Segmenter.
//     This initialization could be time-consuming and shouldn't be done at each call of `segment_str`.
//     In this case, you may want to store the initialized instance in a lazy static like below and call it in `segment_str`.
//     Otherwise, just remove below lines.
//
// Put this import at the top of the file.
use once_cell::sync::Lazy;
//
static SEGMENTER: Lazy<WordSegmenter> = Lazy::new(|| {
    let blob = include_bytes!("../../dictionaries/bin/icu4x-khmer-keys");

    let buffer_provider: BlobDataProvider =
        BlobDataProvider::try_new_from_static_blob(blob).expect("failed to load khmer keys");

    WordSegmenter::try_new_dictionary_with_buffer_provider(&buffer_provider)
        .expect("failed to initialize khmer word segmenter")
});

// Make a small documentation of the specialized Segmenter like below.
/// <Script/Language> specialized [`Segmenter`].
///
/// This Segmenter uses [`<UsedLibraryToSegment>`] internally to segment the provided text.
/// <OptionalAdditionnalExplanations>
//
//TIP: Name the Segmenter with its purpose and not its internal behavior:
//     prefer JapaneseSegmenter (based on the Language) instead of LinderaSegmenter (based on the used Library).
//     Same for the filename, prefer `japanese.rs` instead of `lindera.rs`.
pub struct KhmerSegmenter;

// All specialized segmenters only need to implement the method `segment_str` of the `Segmenter` trait.
impl Segmenter for KhmerSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        let (_, positions) =
            SEGMENTER.segment_str(to_segment).fold((None, vec![]), |mut acc, elem| {
                if acc.0.is_some() {
                    acc.1.push((acc.0.unwrap(), elem));
                }

                acc.0 = Some(elem);

                acc
            });

        // Return the created iterator wrapping it in a Box.
        Box::new(
            positions
                .iter()
                .map(|(start, end)| &to_segment[*start..*end])
                .collect::<Vec<&str>>()
                .into_iter(),
        )
    }
}

// Publish the newly implemented Segmenter:
//	   - import module by adding `mod dummy;` (filename) in `segmenter/mod.rs`
//	   - publish Segmenter by adding `pub use dummy::KhmerSegmenter;` in `segmenter/mod.rs`
//     - running `cargo doc --open` you should see your Segmenter in the segmenter module

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str = "សួស្តីពិភពលោក";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &["សួស្តី", "ពិភពលោក"];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = &["សួស្តី", "ពិភពលោក"];

    // Macro that run several tests on the Segmenter.
    test_segmenter!(KhmerSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Khmer, Language::Khm);
}

// Include the newly implemented Segmenter in the tokenization pipeline:
//	   - assign Segmenter to a Script and a Language by adding it in `SEGMENTERS` in `segmenter/mod.rs`
//	   - check if it didn't break any test or benhchmark

// Your Segmenter will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
