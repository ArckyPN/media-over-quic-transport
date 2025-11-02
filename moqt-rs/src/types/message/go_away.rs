use url::Url;
use varint::{VarInt, x};

// TODO
pub enum GoawayError {}

/// ## Goaway
///
/// A Goaway message signals that a Session is soon
/// ending. It may contain an migration URL to allow
/// recipients to initiate a session migration.
#[derive(Debug, VarInt, PartialEq, Clone)]
#[varint::draft_ref(v = 14)]
pub struct Goaway {
    /// ## Migration URL
    ///
    /// This URL is used to initiate a session
    /// migration.
    ///
    /// However, it is possible to be empty. In
    /// this case no migration can be performed.
    #[varint(length = x(i))]
    pub url: x!(..),
}

impl Goaway {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<x!(..)>,
    {
        Self::from(url)
    }

    pub fn migration_url(&self) -> Result<Option<Url>, GoawayError> {
        // TODO not specified but should probably be valid utf-8
        // TODO larger than 8192 bytes => protocol violation
        todo!("# TODO validate url and return url is it exists (length could be 0 with no data)")
    }
}

impl<T> From<T> for Goaway
where
    T: Into<x!(..)>,
{
    fn from(value: T) -> Self {
        Self { url: value.into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::{TestData, varint_struct_test};

    use super::*;

    impl TestData for Goaway {
        fn test_data() -> Vec<(Self, Vec<u8>, usize)> {
            let v1 = Self::new("http:");
            let b1 = [
                vec![5],           // uri length
                b"http:".to_vec(), // uri
            ]
            .concat();
            let l1 = b1.len() * 8;

            vec![(v1, b1, l1)]
        }
    }

    varint_struct_test!(Goaway);
}
