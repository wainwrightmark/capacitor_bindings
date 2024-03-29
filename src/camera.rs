use std::collections::BTreeMap;

use crate::extern_functions::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::helpers::*;
use crate::{error::Error};

pub struct Camera;

impl Camera {
    /// Prompt the user to pick a photo from an album, or take a new photo with the camera.
    pub async fn get_photo(options: impl Into<ImageOptions>) -> Result<Photo, Error> {
        run_value_value(options, camera_get_photo).await
    }
    /// Allows the user to pick multiple pictures from the photo gallery. On iOS 13 and older it only allows to pick one picture.
    pub async fn pick_images(
        options: impl Into<GalleryImageOptions>,
    ) -> Result<GalleryPhotos, Error> {
        run_value_value(options, camera_pick_images).await
    }

    /// Allows the user to pick multiple pictures from the photo gallery. On iOS 13 and older it only allows to pick one picture.
    pub async fn pick_limited_library_photos() -> Result<GalleryPhotos, Error> {
        run_unit_value(camera_pick_limited_library_photos).await
    }

    #[cfg(feature = "ios")]
    /// iOS 14+ Only: Return an array of photos selected from the limited photo library.
    pub async fn get_limited_library_photos() -> Result<GalleryPhotos, Error> {
        run_unit_value(camera_get_limited_library_photos).await
    }

    /// Check camera and photo album permissions
    pub async fn check_permissions() -> Result<PermissionStatus, Error> {
        run_unit_value(camera_check_permissions).await
    }

    #[cfg(any(feature = "ios", feature = "android"))]
    /// Request camera and photo album permissions. Not implemented on web
    pub async fn request_permissions(
        options: impl Into<CameraPluginPermissions>,
    ) -> Result<PermissionStatus, Error> {
        run_value_value(options, camera_request_permissions).await
    }
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Photo {
    /// The base64 encoded string representation of the image, if using CameraResultType.Base64.
    pub base64_string: Option<String>,
    /// The url starting with 'data:image/jpeg;base64,' and the base64 encoded string representation of the image, if using CameraResultType.DataUrl.
    pub data_url: Option<String>,
    /// If using CameraResultType.Uri, the path will contain a full, platform-specific file URL that can be read later using the Filesystem API.
    pub path: Option<String>,
    /// webPath returns a path that can be used to set the src attribute of an image for efficient loading and rendering.
    pub web_path: Option<String>,
    /// Exif data, if any, retrieved from the image
    pub exif: Option<ExifData>,
    /// The format of the image, ex: jpeg, png, gif. iOS and Android only support jpeg. Web supports jpeg and png. gif is only supported if using file input.
    pub format: String,
    /// Whether if the image was saved to the gallery or not. On Android and iOS, saving to the gallery can fail if the user didn't grant the required permissions. On Web there is no gallery, so always returns false.
    pub saved: Option<bool>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptions {
    /// The quality of image to return as JPEG, from 0-100
    pub quality: u8,
    /// Whether to allow the user to crop or make small edits (platform specific). On iOS 14+ it's only supported for CameraSource.Camera, but not for CameraSource.Photos.
    pub allow_editing: bool,
    /// How the data should be returned. Currently, only 'Base64', 'DataUrl' or 'Uri' is supported
    pub result_type: CameraResultType,
    /// Whether to save the photo to the gallery. If the photo was picked from the gallery, it will only be saved if edited.
    pub save_to_gallery: bool,

    /// The desired maximum width of the saved image. The aspect ratio is respected.
    pub width: u32,
    /// The desired maximum height of the saved image. The aspect ratio is respected.
    pub height: u32,
    /// Whether to automatically rotate the image "up" to correct for orientation in portrait mode
    pub correct_orientation: bool,
    /// The source to get the photo from. By default this prompts the user to select either the photo album or take a photo.
    pub source: CameraSource,
    /// iOS and Web only: The camera direction.
    pub direction: Option<CameraDirection>,

    /// iOS only: The presentation style of the Camera.
    pub presentation_style: Option<PresentationStyle>,
    /// Web only: Whether to use the PWA Element experience or file input. The default is to use PWA Elements if installed and fall back to file input. To always use file input, set this to true. Learn more about PWA Elements: https://capacitorjs.com/docs/web/pwa-elements
    pub web_use_input: Option<bool>,

    /// Text value to use when displaying the prompt.
    pub prompt_label_header: String,
    /// Text value to use when displaying the prompt. iOS only: The label of the 'cancel' button.
    pub prompt_label_cancel: Option<String>,
    /// Text value to use when displaying the prompt. The label of the button to select a saved image.
    pub prompt_label_photo: Option<String>,
    /// Text value to use when displaying the prompt. The label of the button to open the camera.
    pub prompt_label_picture: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GalleryPhotos {
    /// Array of all the picked photos.
    pub photos: Vec<GalleryPhoto>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GalleryPhoto {
    /// Full, platform-specific file URL that can be read later using the Filesystem API.
    pub path: String,
    /// webPath returns a path that can be used to set the src attribute of an image for efficient loading and rendering.
    pub web_path: Option<String>,
    /// Exif data, if any, retrieved from the image
    pub exif: Option<ExifData>,
    /// The format of the image, ex: jpeg, png, gif. iOS and Android only support jpeg. Web supports jpeg, png and gif.
    pub format: String,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ExifData {
    #[serde(flatten)]
    pub data: BTreeMap<String, String>,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GalleryImageOptions {
    /// The quality of image to return as JPEG, from 0-100
    pub quality: u8,
    /// The desired maximum width of the saved image. The aspect ratio is respected.
    pub width: u32,
    /// The desired maximum height of the saved image. The aspect ratio is respected.
    pub height: u32,
    /// Whether to automatically rotate the image "up" to correct for orientation in portrait mode
    pub correct_orientation: bool,
    /// iOS only: The presentation style of the Camera.
    pub presentation_style: Option<PresentationStyle>,
    /// iOS only: Maximum number of pictures the user will be able to choose.
    pub limit: Option<u32>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionStatus {
    pub camera: CameraPermissionState,
    pub photos: CameraPermissionState,
}

#[skip_serializing_none]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct CameraPluginPermissions {
    pub permissions: Vec<CameraPermissionType>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CameraPermissionState {
    Prompt,
    PromptWithRationale,
    Granted,
    Denied,
    Limited,
}

impl From<PermissionState> for CameraPermissionState {
    fn from(value: PermissionState) -> Self {
        match value {
            PermissionState::Prompt => CameraPermissionState::Prompt,
            PermissionState::PromptWithRationale => CameraPermissionState::PromptWithRationale,
            PermissionState::Granted => CameraPermissionState::Granted,
            PermissionState::Denied => CameraPermissionState::Denied,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionState {
    Prompt,
    PromptWithRationale,
    Granted,
    Denied,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresentationStyle {
    Fullscreen,
    Popover,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CameraPermissionType {
    Camera,
    Photos,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CameraResultType {
    Uri,
    Base64,
    DataUrl,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CameraSource {
    /// Prompts the user to select either the photo album or take a photo.
    Prompt,
    /// Take a new photo using the camera.
    Camera,
    /// Pick an existing photo from the gallery or photo album.
    Photos,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CameraDirection {
    Rear,
    Front,
}
