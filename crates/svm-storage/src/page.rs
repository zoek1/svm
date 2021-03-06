/// A page is `4096 bytes`
pub const PAGE_SIZE: usize = 4096;

/// A `PageIndex` is a one-dimensional tuple of `(u32)` representing a page-index (non-negative integer)
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub struct PageIndex(pub u32);

/// `PageHash` length is 32 bytes
pub const PAGE_HASH_LEN: usize = 32;

/// A `PageHash` is a one-dimensional tuple of `([u8; PAGE_HASH_LEN])` representing hash of the page-content.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PageHash(pub [u8; PAGE_HASH_LEN]);

impl AsRef<[u8]> for PageHash {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&[u8]> for PageHash {
    fn from(slice: &[u8]) -> PageHash {
        assert_eq!(
            PAGE_HASH_LEN,
            slice.len(),
            "`PageHash::from` expects exactly 32 bytes input",
        );

        let mut bytes = [0; PAGE_HASH_LEN];
        bytes.copy_from_slice(slice);

        PageHash(bytes)
    }
}

/// A `Page` consists of a tuple of `(PageIndex, PageHash, Vec<u8>`)`
///
/// `PageIndex` - The page indexes within the Smart Contract
/// `PageHash`  - Hash of the page. Derived from `PageIndex` + `Page Data`.
///               See also: `PageHasher` under `traits`
/// `Vec<u8>`   - The page data
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Page(pub PageIndex, pub PageHash, pub Vec<u8>);

/// A `SliceIndex` is a one-dimensional tuple of `(u32)`
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SliceIndex(pub u32);

/// Defines a page-slice memory
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PageSliceLayout {
    /// The slice index
    pub slice_idx: SliceIndex,

    /// The page index the slices belong to
    pub page_idx: PageIndex,

    /// The page offset where the slice starts
    pub offset: u32,

    /// The length of the slice in bytes
    pub len: u32,
}

/// Allocates a new page (`Vec<u8>`) consisting only of zeros
#[inline(always)]
pub fn zero_page() -> Vec<u8> {
    vec![0; PAGE_SIZE]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "`PageHash::from` expects exactly 32 bytes input")]
    fn page_hash_expects_exactly_32_bytes_input() {
        PageHash::from([0; 10].as_ref());
    }

    #[test]
    fn page_hash_from_slice() {
        let raw: [u8; 32] = [
            01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33, 44,
            55, 66, 77, 88, 99, 251, 252, 253, 254, 255,
        ];

        let ph = PageHash::from(raw.as_ref());

        assert_eq!(
            PageHash([
                01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 20, 30, 40, 50, 60, 70, 80, 90, 11, 22, 33,
                44, 55, 66, 77, 88, 99, 251, 252, 253, 254, 255
            ]),
            ph
        );
    }
}
