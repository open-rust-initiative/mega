//!
//!
//!
//!
//!
mod object;
mod pack;

use std::fmt::Display;

use crate::git::errors::GitError;

/// In Git, each object type is assigned a unique integer value, which is used to identify the
/// type of the object in Git repositories.
///
/// * `Blob` (1): A Git object that stores the content of a file.
/// * `Tree` (2): A Git object that represents a directory or a folder in a Git repository.
/// * `Commit` (3): A Git object that represents a commit in a Git repository, which contains
/// information such as the author, committer, commit message, and parent commits.
/// * `Tag` (4): A Git object that represents a tag in a Git repository, which is used to mark a
/// specific point in the Git history.
/// * `OffsetDelta` (6): A Git object that represents a delta between two objects, where the delta
/// is stored as an offset to the base object.
/// * `HashDelta` (7): A Git object that represents a delta between two objects, where the delta
/// is stored as a hash of the base object.
///
/// By assigning unique integer values to each Git object type, Git can easily and efficiently
/// identify the type of an object and perform the appropriate operations on it. when parsing a Git
/// repository, Git can use the integer value of an object's type to determine how to parse
/// the object's content.
#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Debug, Clone, Copy)]
pub enum ObjectType {
    Commit,
    Tree,
    Blob,
    Tag,
    OffsetDelta,
    HashDelta,
}

/// Display trait for Git objects type
impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
            ObjectType::Tag => write!(f, "tag"),
            ObjectType::OffsetDelta => write!(f, "OffsetDelta"),
            ObjectType::HashDelta => write!(f, "HashDelta"),
        }
    }
}

impl ObjectType {
    /// Parses a string representation of a Git object type and returns an ObjectType value
    /// # Examples
    /// ```
    ///     let blob = ObjectType::from_string("blob").unwrap();
    /// ```
    #[allow(unused)]
    pub fn from_string(s: &str) -> Result<ObjectType, GitError> {
        match s {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            "commit" => Ok(ObjectType::Commit),
            "tag" => Ok(ObjectType::Tag),
            _ => Err(GitError::InvalidObjectType(s.to_string())),
        }
    }

    /// Convert an object type to a byte array.
    /// # Examples
    /// ```
    ///     let blob = ObjectType::Blob;
    ///     let blob_bytes = blob.to_bytes().unwrap();
    ///     assert_eq!(blob_bytes, vec![0x62, 0x6c, 0x6f, 0x62]);
    /// ```
    #[allow(unused)]
    pub fn to_data(self) -> Result<Vec<u8>, GitError> {
        match self {
            ObjectType::Blob => Ok(vec![0x62, 0x6c, 0x6f, 0x62]),
            ObjectType::Tree => Ok(vec![0x74, 0x72, 0x65, 0x65]),
            ObjectType::Commit => Ok(vec![0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74]),
            ObjectType::Tag => Ok(vec![0x74, 0x61, 0x67]),
            _ => Err(GitError::InvalidObjectType(self.to_string())),
        }
    }

    /// Convert an object type to a number.
    /// # Examples
    /// ```
    ///    let blob = ObjectType::Blob;
    ///    let blob_number = blob.type2number();
    ///    assert_eq!(blob_number, 1);
    /// ```
    #[allow(unused)]
    pub fn type2number(&self) -> u8 {
        match self {
            ObjectType::Blob => 1,
            ObjectType::Tree => 2,
            ObjectType::Commit => 3,
            ObjectType::Tag => 4,
            ObjectType::OffsetDelta => 6,
            ObjectType::HashDelta => 7,
        }
    }

    /// Convert a number to an object type.
    /// # Examples
    /// ```
    ///   let blob_number = 1;
    ///   let blob = ObjectType::number2type(blob_number).unwrap();
    ///   assert_eq!(blob, ObjectType::Blob);
    /// ```
    #[allow(unused)]
    pub fn number2type(number: u8) -> Result<ObjectType, GitError> {
        match number {
            1 => Ok(ObjectType::Blob),
            2 => Ok(ObjectType::Tree),
            3 => Ok(ObjectType::Commit),
            4 => Ok(ObjectType::Tag),
            6 => Ok(ObjectType::OffsetDelta),
            7 => Ok(ObjectType::HashDelta),
            _ => Err(GitError::InvalidObjectType(format!("Invalid object type number: {}", number))),
        }
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_object_type_to_data() {
        let blob = super::ObjectType::Blob;
        let blob_bytes = blob.to_data().unwrap();
        assert_eq!(blob_bytes, vec![0x62, 0x6c, 0x6f, 0x62]);
    }

    #[test]
    fn test_object_type_from_string() {
        let tree = super::ObjectType::from_string("tree").unwrap();
        assert_eq!(tree, super::ObjectType::Tree);
    }

    #[test]
    fn test_object_type_type2number() {
        let commit = super::ObjectType::Commit;
        let commit_number = commit.type2number();
        assert_eq!(commit_number, 3);
    }

    #[test]
    fn test_object_type_number2type() {
        let tag_number = 4;
        let tag = super::ObjectType::number2type(tag_number).unwrap();
        assert_eq!(tag, super::ObjectType::Tag);
    }
}