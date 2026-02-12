use std::io;

use io::Seek;
use io::Write;

use io::Read;

use image::DynamicImage;
use image::ImageFormat;

use image::imageops::FilterType;

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum FilterMode {
    Nearest,
    Triangle,
    #[default]
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl From<FilterMode> for FilterType {
    fn from(typ: FilterMode) -> Self {
        match typ {
            FilterMode::Nearest => Self::Nearest,
            FilterMode::Triangle => Self::Triangle,
            FilterMode::CatmullRom => Self::CatmullRom,
            FilterMode::Gaussian => Self::Gaussian,
            FilterMode::Lanczos3 => Self::Lanczos3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum AspectMode {
    #[default]
    Preserve,
    Ignore,
    Clip,
}

#[derive(Debug, Clone, Copy)]
pub struct ResizeConfig {
    pub new_width: u32,
    pub new_height: u32,
}

impl ResizeConfig {
    pub fn new_square(wh: u32) -> Self {
        Self {
            new_width: wh,
            new_height: wh,
        }
    }
}

impl ResizeConfig {
    pub fn from_size_hint(size_hint: &str) -> Option<Self> {
        match size_hint {
            "min" | "minimal" => Some(Self::new_square(8)),
            "tiny" => Some(Self::new_square(16)),
            "small" => Some(Self::new_square(32)),
            "normal" => Some(Self::new_square(64)),
            "large" => Some(Self::new_square(128)),
            "Large" => Some(Self::new_square(256)),
            "LARGE" => Some(Self::new_square(512)),
            "huge" => Some(Self::new_square(1024)),
            "Huge" => Some(Self::new_square(2048)),
            "HUGE" => Some(Self::new_square(4096)),
            _ => None,
        }
    }
}

impl AspectMode {
    pub fn resize_image(
        &self,
        original: &DynamicImage,
        cfg: ResizeConfig,
        filter: FilterMode,
    ) -> DynamicImage {
        let typ: FilterType = filter.into();
        let w: u32 = cfg.new_width;
        let h: u32 = cfg.new_height;
        match self {
            Self::Preserve => original.resize(w, h, typ),
            Self::Ignore => original.resize_exact(w, h, typ),
            Self::Clip => original.resize_to_fill(w, h, typ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShrinkConfig {
    pub filter: FilterMode,
    pub aspect: AspectMode,
    pub new_sz: ResizeConfig,
}

impl ShrinkConfig {
    pub fn convert(&self, original: &DynamicImage) -> DynamicImage {
        self.aspect.resize_image(original, self.new_sz, self.filter)
    }
}

pub fn bytes2image(dat: &[u8]) -> Result<DynamicImage, io::Error> {
    image::load_from_memory(dat).map_err(io::Error::other)
}

pub fn img2wtr<W>(img: &DynamicImage, wtr: W) -> Result<(), io::Error>
where
    W: Write + Seek,
{
    img.write_to(wtr, ImageFormat::Pnm)
        .map_err(io::Error::other)
}

pub fn reader2bytes<R>(rdr: R, limit: u64) -> Result<Vec<u8>, io::Error>
where
    R: Read,
{
    let mut taken = rdr.take(limit);
    let mut buf = vec![];
    taken.read_to_end(&mut buf)?;
    Ok(buf)
}
