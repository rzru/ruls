use std::path::PathBuf;
use std::fs::symlink_metadata;

pub trait ExtendPathInfo {
    fn is_symlink_local(&self) -> bool;
    fn is_in_array(&self, values: Vec<&str>) -> bool;
    fn is_image(&self) -> bool;
    fn is_archive(&self) -> bool;
}

impl ExtendPathInfo for PathBuf {
    fn is_symlink_local(&self) -> bool {
        if let Ok(metadata) = symlink_metadata(self) {
            let file_type = metadata.file_type();

            return file_type.is_symlink();
        }

        false
    }

    fn is_in_array(&self, values: Vec<&str>) -> bool {
        if let Some(extension) = self.extension() {
            if let Some(extension) = extension.to_str() {
                let extension = &extension.to_lowercase()[..];
                return values.contains(&extension)
            }
        }

        false
    }

    fn is_image(&self) -> bool {
        let image_extensions = vec!["tif", "tiff", "jpg", "bmp", "svg", "bmp", "jpeg", "gif", "eps", "webp", "psd", "raw", "heif", "indd", "ai"];
        self.is_in_array(image_extensions)
    }

    fn is_archive(&self) -> bool {
        let archive_extensions = vec!["7z", "zip", "rar", "bz2", "gz", "deb", "gzip"];
        self.is_in_array(archive_extensions)
    }
}
