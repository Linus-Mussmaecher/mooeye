use std::{collections::HashMap, ffi::OsStr, path::Path, time::Duration};

use ggez::{
    graphics::{Drawable, Image, Rect},
    *,
};

use crate::{
    ui_element::{Size, UiElementBuilder},
    UiContent,
};
use std::hash::Hash;

use regex;

/// A Sprite is an advanced version of an image element, displaying an animated picture that can have multiple states (e.g. a walking, attacking, etc. version of a player character)
/// The sprite is initalized using an image file that contains multiple rows of images (each row representing a variant), where each row contains the same number of animation frames for each variant.
/// Drawing the sprite repeatedly draws every frame of the selected variant in order and then repeats from the beginning.
#[derive(Debug, Clone)]
pub struct Sprite {
    /// Width of one sprite in the underlying sprite sheet.
    w: u32,
    /// Height of one sprite in the underlying sprite sheet.
    h: u32,
    /// The underlying sprite sheet.
    spritesheet: Image,

    /// The target time to spend each frame.
    frame_time: Duration,
    /// Time spent in the current frame.
    current_frame_time: Duration,
    /// The current frame.
    current_frame: u32,
    /// The current variant.
    current_variant: u32,
}

impl Sprite {
    /// Create a new sprite using the passed [ggez::graphics::Image] and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image within the sheet have to be passed manually.
    /// Will never fail, as the image is already loaded by ggez.
    pub fn new(spritesheet: Image, w: u32, h: u32, frame_time: Duration) -> Self {
        Self {
            frame_time,
            w,
            h,
            spritesheet,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        }
    }

    /// Create a new sprite from the file found at the passed path and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image within the sheet have to be passed manually.
    /// May fail if the image cannot be loaded, because f.e. the path is wrong. Passing 'wrong' size values will yield unexpected behaviour but not panic.
    pub fn from_path(
        path: impl AsRef<Path>,
        ctx: &Context,
        w: u32,
        h: u32,
        frame_time: Duration,
    ) -> Result<Self, GameError> {
        Ok(Self {
            frame_time,
            w,
            h,
            spritesheet: Image::from_path(ctx, path)?,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        })
    }

    /// Create a new sprite using from the file found at the passed path and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image are read from the file name.
    /// The file name needs to be formatted as ```name_possibly_containing_underscores_width_height.extension```.
    /// May fail if the image cannot be loaded (e.g. if the path is wrong) or the file name is not formatted correctly.
    pub fn from_path_fmt(
        path: impl AsRef<Path>,
        ctx: &Context,
        frame_time: Duration,
    ) -> Result<Self, GameError> {
        let pathstring = path
            .as_ref()
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .ok_or(GameError::CustomError(
                "Path formatted incorrectly.".to_owned(),
            ))?;

        let width_height = pathstring
            .split('.')
            .next()
            .unwrap_or("")
            .split("_")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .take(2)
            .map(|s| *s)
            .rev()
            .collect::<Vec<&str>>();

        let w = *width_height.get(0).ok_or(GameError::CustomError(format!(
            "Filename formatted incorretly - not ending in _width_height.extension. Filename: {}",
            pathstring
        )))?;
        let h = *width_height.get(1).ok_or(GameError::CustomError(format!(
            "Filename formatted incorretly - not ending in _width_height.extension. Filename: {}",
            pathstring
        )))?;
        let w = w.parse::<u32>().map_err(|_| {
            GameError::CustomError(
                format!("Filename formatted correctly, but width numbers could not be parsed. Width number: {}", w),
            )
        })?;
        let h = h.parse::<u32>().map_err(|_| {
            GameError::CustomError(
                format!("Filename formatted correctly, but height numbers could not be parsed. Height number: {}", h),
            )
        })?;

        Ok(Self {
            frame_time,
            w,
            h,
            spritesheet: Image::from_path(ctx, path)?,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        })
    }

    /// Sets the variant this sprite is currently displaying. Numbers that are too large to represent a valid variant will wrap around.
    pub fn set_variant(&mut self, variant: u32) {
        self.current_variant = variant % (self.spritesheet.height() / self.h);
        self.current_frame_time = Duration::ZERO;
        self.current_frame = 0;
    }

    /// Returns the variant this sprite is currently displaying.
    pub fn get_variant(&self) -> u32 {
        self.current_variant
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.w as f32, self.h as f32)
    }

    pub fn get_frame_time(&self) -> Duration {
        self.frame_time
    }

    pub fn get_cycle_time(&self) -> Duration {
        self.frame_time * self.spritesheet.width() / self.w
    }

    /// Draws this sprite as given by the paramters, advancing the displayed frame as needed.
    pub fn draw_sprite(
        &mut self,
        ctx: &Context,
        canvas: &mut graphics::Canvas,
        param: impl Into<graphics::DrawParam>,
    ) {
        self.current_frame_time += ctx.time.delta();
        while self.current_frame_time >= self.frame_time {
            self.current_frame_time -= self.frame_time;
            self.current_frame = (self.current_frame + 1) % (self.spritesheet.width() / self.w);
        }

        self.draw(canvas, param);
    }
}

impl Drawable for Sprite {
    fn draw(&self, canvas: &mut graphics::Canvas, param: impl Into<graphics::DrawParam>) {
        self.spritesheet.draw(
            canvas,
            (param.into() as graphics::DrawParam).src(Rect::new(
                (self.w * self.current_frame) as f32 / self.spritesheet.width() as f32,
                (self.h * self.current_variant) as f32 / self.spritesheet.height() as f32,
                self.w as f32 / self.spritesheet.width() as f32,
                self.h as f32 / self.spritesheet.height() as f32,
            )),
        );
    }

    fn dimensions(
        &self,
        _gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        Some(Rect::new(0., 0., self.w as f32, self.h as f32))
    }
}

impl<T: Copy + Eq + Hash> UiContent<T> for Sprite {
    fn to_element_builder(self, id: u32, _ctx: &Context) -> UiElementBuilder<T>
    where
        Self: Sized + 'static,
    {
        let (w, h) = (self.w, self.h);
        UiElementBuilder::new(id, self)
            .with_size(
                Size::Fill(w as f32, f32::INFINITY),
                Size::Fill(h as f32, f32::INFINITY),
            )
            .with_preserve_ratio(true)
    }

    fn draw_content(
        &mut self,
        ctx: &mut Context,
        canvas: &mut graphics::Canvas,
        param: crate::ui_element::UiDrawParam,
    ) {
        self.draw_sprite(
            ctx,
            canvas,
            param.param.dest_rect(Rect::new(
                param.target.x,
                param.target.y,
                param.target.w / self.w as f32,
                param.target.h / self.h as f32,
            )),
        );
    }
}

/// A pool that contains a number of initialized [sprite::Sprite]s at once and can be passed around and allows initialization of sprites using the prototype pattern and without having to re-access the file system or pass around a loading context.
/// Provides functions for quickly initalizing folders of sprites and access methods similar to those of [graphics::Image] and [mooeye::sprite::Sprite].
/// ### File format and access
/// File names must be formatted as ```NAME_WIDTH_HEIGHT.EXTENSION```.
/// Access keys into the pool are the full path to the image file (relative to the resource directory), followed by only NAME, width, height and extension are stripped.
/// This allows easy replacement of sprites with different formats.
/// ### Example
/// A file named ```mage_16_16.png``` in a subfolder ```/sprites/player``` of the resource folder will be accessible with the key ```/sprites/player/mage```.
pub struct SpritePool {
    sprites: HashMap<String, Sprite>,
}

impl SpritePool {
    /// Creates a new (empty) [SpritePool] instance.
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }

    /// Loads all sprites within the given folder (relative to the ggez resource directory, see [ggez::context::ContextBuilder]) into the sprite pool.
    /// Can also search all subfolders.
    /// See [SpritePool] for required name formatting in order to load sprites correctly.
    pub fn with_folder(
        mut self,
        ctx: &Context,
        path: impl AsRef<Path>,
        search_subfolders: bool,
    ) -> Self {
        let paths = ctx
            .fs
            .read_dir(path.as_ref())
            .expect("Could not find specified path.");

        let sprite_match = regex::Regex::new(r"(.*)_\d*_\d*.[png|jpg|jpeg]").unwrap();

        for sub_path in paths {
            let path_string = sub_path.to_string_lossy().to_string();
            if sprite_match.is_match(&path_string) {
                if let Ok(sprite) =
                    Sprite::from_path_fmt(sub_path.clone(), ctx, Duration::from_secs_f32(0.25))
                {
                    self.sprites.insert(
                        sprite_match
                            .captures(&path_string)
                            .map(|c| c.get(1).map(|m| m.as_str()))
                            .unwrap_or_default()
                            .unwrap_or_default()
                            .replace(r"\", "/"),
                        sprite,
                    );
                }
            } else if search_subfolders {
                self = self.with_folder(ctx, sub_path, search_subfolders);
            }
        }
        //println!("Now containing {} files.", self.sprites.len());
        self
    }

    /// Initialies a sprite from the sprite pool.
    /// The path syntax is exactly the same as for initalizing images or sprites, relative to the ggez resource folder.
    /// See [graphics::Image] and [sprite::Sprite].
    /// If the sprite (path) is not yet contained in the pool, an error is returned.
    /// For lazy initalization, use [init_sprite_lazy] instead.
    /// See [SpritePool] for rules related to key assignment.
    pub fn init_sprite(
        &self,
        path: impl AsRef<Path>,
        frame_time: Duration,
    ) -> Result<Sprite, GameError> {
        let sprite = self
            .sprites
            .get(&path.as_ref().to_string_lossy().to_string())
            .ok_or_else(|| GameError::CustomError("Could not find sprite.".to_owned()))?;
        Ok(Sprite {
            frame_time,
            ..sprite.clone()
        })
    }

    /// Initialies a sprite from the sprite pool.
    /// The path syntax is exactly the same as for initalizing images or sprites, relative to the ggez resource folder.
    /// See [graphics::Image] and [sprite::Sprite].
    /// If the sprite (path) is not yet contained in the pool, the system will attempt to load it from the file system and return it.
    /// If this also fails, an error is returned.
    /// See [SpritePool] for rules related to key assignment.
    pub fn init_sprite_lazy(
        &mut self,
        ctx: &Context,
        path: impl AsRef<Path>,
        frame_time: Duration,
    ) -> Result<Sprite, GameError> {
        let key = &path.as_ref().to_string_lossy().to_string();
        if !self.sprites.contains_key(key) {
            let sprite = Sprite::from_path_fmt(path.as_ref(), ctx, Duration::ZERO)?;
            self.sprites.insert((*key).clone(), sprite);
        }
        self.init_sprite(path, frame_time)
    }

    /// Prints all currently registered keys of this sprite pool. Useful if you are debugging key-issues.
    pub fn print_keys(&self) {
        println!("Currently registered keys");
        for (key, _) in self.sprites.iter() {
            println!(" | {}", &key);
        }
        println!("-+----------------")
    }
}
