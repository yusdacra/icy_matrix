use directories_next::ProjectDirs;
use http::Uri;
pub use iced::image::Handle as ImageHandle;
use iced_native::image::Data;
use indexmap::IndexMap;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

const CONTENT_DIR_NAME: &str = "content";

pub fn infer_type_from_bytes(data: &[u8]) -> String {
    infer::get(&data)
        .map(|filetype| filetype.mime_type().to_string())
        .unwrap_or_else(|| String::from("application/octet-stream"))
}

pub fn get_filename<'a, P>(path: P) -> Cow<'a, str>
where
    P: AsRef<Path> + 'a,
{
    path.as_ref()
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| Cow::from("unknown"))
}

#[derive(Debug, Clone)]
pub struct ContentStore {
    app_dirs: ProjectDirs,
}

impl ContentStore {
    // Hashes the given data with Sha3_256 and returns the hex representation
    pub fn hash_content(&self, data: &[u8]) -> String {
        use sha3::{Digest, Sha3_256};
        hex::encode(Sha3_256::digest(data))
    }

    pub fn content_path(&self, id: &str) -> PathBuf {
        let normalized_id = id.replace(|c| [' ', '/', '\\'].contains(&c), "_");
        self.app_dirs.cache_dir().join(CONTENT_DIR_NAME).join(id)
    }

    pub fn content_mimetype(&self, id: &str) -> String {
        infer::get_from_path(self.content_path(id))
            .map_or(None, Some)
            .flatten()
            .map(|filetype| filetype.mime_type().to_string())
            .unwrap_or_else(|| String::from("application/octet-stream"))
    }

    pub fn content_exists(&self, id: &str) -> bool {
        self.content_path(id).exists()
    }

    pub fn app_dirs(&self) -> &ProjectDirs {
        &self.app_dirs
    }
}

fn get_image_size_from_handle(handle: &ImageHandle) -> Option<u64> {
    // This one angers me a lot, iced pls read the file beforehand and cache it
    match handle.data() {
        Data::Bytes(raw) => Some(raw.len() as u64),
        Data::Path(path) => std::fs::metadata(path).map_or(None, |meta| Some(meta.len())),
        Data::Pixels {
            pixels,
            height: _,
            width: _,
        } => Some(pixels.len() as u64),
    }
}

pub struct ThumbnailCache {
    thumbnails: IndexMap<Uri, ImageHandle>,
    max_size: u64,
}

impl Default for ThumbnailCache {
    fn default() -> Self {
        const MAX_CACHE_SIZE: u64 = 1000 * 1000 * 100; // 100Mb
        Self::new(MAX_CACHE_SIZE)
    }
}

impl ThumbnailCache {
    pub fn new(max_size: u64) -> Self {
        Self {
            thumbnails: IndexMap::new(),
            max_size,
        }
    }

    pub fn put_thumbnail(&mut self, thumbnail_url: Uri, thumbnail: ImageHandle) {
        let thumbnail_size = match get_image_size_from_handle(&thumbnail) {
            Some(size) => size,
            None => return,
        };
        let cache_size = self.len();

        if cache_size + thumbnail_size > self.max_size {
            let mut current_size = 0;
            let mut remove_upto = 0;
            for (index, size) in self
                .thumbnails
                .values()
                .flat_map(|h| get_image_size_from_handle(h))
                .enumerate()
            {
                if current_size >= thumbnail_size {
                    remove_upto = index + 1;
                    break;
                }
                current_size += size;
            }
            for index in 0..remove_upto {
                self.thumbnails.shift_remove_index(index);
            }
        } else {
            self.thumbnails.insert(thumbnail_url, thumbnail);
        }
    }

    pub fn len(&self) -> u64 {
        self.thumbnails
            .values()
            .flat_map(|h| get_image_size_from_handle(h))
            .sum()
    }

    pub fn has_thumbnail(&self, thumbnail_url: &Uri) -> bool {
        self.thumbnails.contains_key(thumbnail_url)
    }

    pub fn get_thumbnail(&self, thumbnail_url: &Uri) -> Option<&ImageHandle> {
        self.thumbnails.get(thumbnail_url)
    }

    pub fn invalidate_thumbnail(&mut self, thumbnail_url: &Uri) {
        self.thumbnails.remove(thumbnail_url);
    }
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Image,
    Audio,
    Video,
    Other,
}

impl ContentType {
    pub fn new(mimetype: &str) -> Self {
        use ContentType::*;

        if let Some(filetype) = mimetype.split('/').next() {
            match filetype {
                "image" => Image,
                "audio" => Audio,
                "video" => Video,
                _ => Other,
            }
        } else {
            Other
        }
    }
}

impl From<&str> for ContentType {
    fn from(other: &str) -> Self {
        ContentType::new(other)
    }
}
