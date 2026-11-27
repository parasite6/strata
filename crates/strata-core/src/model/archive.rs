// SPDX-License-Identifier: MIT

use super::FileEntry;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchiveFormat {
    Zip,
    SevenZ,
    TarGz,
    Tar,
    Rar,
}

impl ArchiveFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Zip => "zip",
            Self::SevenZ => "7z",
            Self::TarGz => "tar.gz",
            Self::Tar => "tar",
            Self::Rar => "rar",
        }
    }

    pub fn supports_password(self) -> bool {
        matches!(self, Self::Zip | Self::SevenZ | Self::Rar)
    }

    pub fn supports_level(self) -> bool {
        matches!(self, Self::Zip | Self::SevenZ | Self::TarGz)
    }

    pub fn can_encrypt_names(self) -> bool {
        self == Self::SevenZ
    }

    pub fn from_extension(name: &str) -> Option<Self> {
        let lower = name.to_ascii_lowercase();
        if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
            Some(Self::TarGz)
        } else if lower.ends_with(".tar") {
            Some(Self::Tar)
        } else if lower.ends_with(".zip") || lower.ends_with(".skill") {
            Some(Self::Zip)
        } else if lower.ends_with(".7z") {
            Some(Self::SevenZ)
        } else if cfg!(feature = "rar") && lower.ends_with(".rar") {
            Some(Self::Rar)
        } else {
            None
        }
    }

    pub fn for_entry(entry: &FileEntry) -> Option<Self> {
        if !entry.is_file() || entry.location.native_path().is_none() {
            return None;
        }
        Self::from_extension(&entry.display_name)
    }
}

/// How hard an archive's encoder works. Each level maps to the 0–9 scale shared by
/// DEFLATE, gzip and LZMA2.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CompressionLevel {
    Store,
    Fastest,
    Fast,
    #[default]
    Normal,
    Maximum,
    Ultra,
}

impl CompressionLevel {
    pub const ALL: [Self; 6] = [
        Self::Store,
        Self::Fastest,
        Self::Fast,
        Self::Normal,
        Self::Maximum,
        Self::Ultra,
    ];

    pub fn value(self) -> u32 {
        match self {
            Self::Store => 0,
            Self::Fastest => 1,
            Self::Fast => 3,
            Self::Normal => 6,
            Self::Maximum => 8,
            Self::Ultra => 9,
        }
    }
}

/// What a new archive is written as.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveSettings {
    pub format: ArchiveFormat,
    pub password: Option<String>,
    pub level: CompressionLevel,
    /// Hides member names behind the password where the format can.
    pub encrypt_names: bool,
}

impl ArchiveSettings {
    pub fn new(format: ArchiveFormat) -> Self {
        Self {
            format,
            password: None,
            level: CompressionLevel::default(),
            encrypt_names: true,
        }
    }

    pub fn with_password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
    }

    pub fn with_level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    pub fn with_encrypt_names(mut self, encrypt_names: bool) -> Self {
        self.encrypt_names = encrypt_names;
        self
    }
}
