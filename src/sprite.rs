use std::{collections::HashMap, ffi::OsStr, io::BufReader, path::Path, time::Duration};

use good_web_game::{
    event::GraphicsContext,
    graphics::{Drawable, Image, Rect},
    *,
};

use crate::{
    ui::UiContent,
    ui::{Size, UiElementBuilder},
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
    /// The underlying sprite sheet. Is an option to allow a default.
    spritesheet: Option<Image>,

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
    /// Create a new sprite using the passed [good_web_game::graphics::Image] and set the duration after which a frame change occurs.
    /// The values for the width and height of a single image within the sheet have to be passed manually.
    /// Will never fail, as the image is already loaded by good_web_game.
    pub fn new(spritesheet: Image, w: u32, h: u32, frame_time: Duration) -> Self {
        Self {
            frame_time,
            w,
            h,
            spritesheet: Some(spritesheet),
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
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
        w: u32,
        h: u32,
        frame_time: Duration,
    ) -> Result<Self, GameError> {
        Ok(Self {
            frame_time,
            w,
            h,
            spritesheet: Image::new(ctx, gfx_ctx, path).ok().map(|mut f| {
                f.set_filter(miniquad::FilterMode::Nearest);
                f
            }),
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
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
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
            .split('_')
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .take(2)
            .copied()
            .rev()
            .collect::<Vec<&str>>();

        let w = *width_height.first().ok_or(GameError::CustomError(format!(
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
            spritesheet: Image::new(ctx, gfx_ctx, path).ok().map(|mut f| {
                f.set_filter(miniquad::FilterMode::Nearest);
                f
            }),
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        })
    }

    /// Sets the variant this sprite is currently displaying. Numbers that are too large to represent a valid variant will wrap around.
    pub fn set_variant(&mut self, variant: u32) {
        if self.current_variant != variant {
            self.current_variant = variant;
            if self.h != 0 {
                self.current_variant %= self
                    .spritesheet
                    .as_ref()
                    .map(|img| img.height() as u32)
                    .unwrap_or_default()
                    / self.h;
            }
            self.current_frame_time = Duration::ZERO;
            self.current_frame = 0;
        }
    }

    /// Returns the variant this sprite is currently displaying.
    pub fn get_variant(&self) -> u32 {
        self.current_variant
    }

    /// Returns the width and height of a single frame of this sprite.
    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.w as f32, self.h as f32)
    }

    /// Returns the duration between two frames.
    pub fn get_frame_time(&self) -> Duration {
        self.frame_time
    }

    /// Set the duration to wait between displaying two succeding frames.
    pub fn set_frame_time(&mut self, frame_time: Duration) {
        self.frame_time = frame_time;
    }

    /// Returns the required time to cycle through every frame of the sprite.
    /// Returns 0 if the sprite has a frame time of zero.
    pub fn get_cycle_time(&self) -> Duration {
        self.frame_time
            * self
                .spritesheet
                .as_ref()
                .map(|img| img.width() as u32)
                .unwrap_or_default()
            / self.w
    }

    /// Draws this sprite as given by the paramters, advancing the displayed frame as needed.
    pub fn draw_sprite(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
        param: impl Into<graphics::DrawParam>,
    ) {
        self.current_frame_time += good_web_game::timer::delta(ctx);
        while self.current_frame_time >= self.frame_time && !self.frame_time.is_zero() {
            self.current_frame_time -= self.frame_time;
            self.current_frame = (self.current_frame + 1)
                % (self
                    .spritesheet
                    .as_ref()
                    .map(|img| img.width() as u32)
                    .unwrap_or_default()
                    / self.w);
        }

        self.draw(ctx, gfx_ctx, param.into())
            .expect("[ERROR/Mooeye] Drawing sprite error");
    }
}

impl Drawable for Sprite {
    fn draw(
        &self,
        ctx: &mut Context,
        quad_ctx: &mut miniquad::graphics::GraphicsContext,
        param: graphics::DrawParam,
    ) -> GameResult {
        if let Some(spritesheet) = &self.spritesheet {
            spritesheet.draw(
                ctx,
                quad_ctx,
                param.src(Rect::new(
                    (self.w * self.current_frame) as f32 / spritesheet.width() as f32,
                    (self.h * self.current_variant) as f32 / spritesheet.height() as f32,
                    self.w as f32 / spritesheet.width() as f32,
                    self.h as f32 / spritesheet.height() as f32,
                )),
            )?;
            Ok(())
        } else {
            Err(GameError::UnknownError(
                "[ERROR/Mooeye] Something went wrong when drawing a sprite".to_owned(),
            ))
        }
    }

    fn set_blend_mode(&mut self, _mode: Option<graphics::BlendMode>) {}

    fn blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }

    fn dimensions(&self, _: &mut Context) -> Option<Rect> {
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
        gfx_ctx: &mut good_web_game::event::GraphicsContext,
        param: crate::ui::UiDrawParam,
    ) {
        self.draw_sprite(
            ctx,
            gfx_ctx,
            param
                .param
                .dest(graphics::Point2 {
                    x: param.target.x,
                    y: param.target.y,
                })
                .scale(graphics::Vector2 {
                    x: param.target.w / self.w as f32,
                    y: param.target.h / self.h as f32,
                }),
        );
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            w: 0,
            h: 0,
            spritesheet: None,
            frame_time: Duration::ZERO,
            current_frame_time: Duration::ZERO,
            current_frame: 0,
            current_variant: 0,
        }
    }
}

/// A pool that contains a number of initialized [Sprite]s at once and can be passed around and allows initialization of sprites using the prototype pattern and without having to re-access the file system or pass around a loading context.
/// Provides functions for quickly initalizing folders of sprites and access methods similar to those of [graphics::Image] and [Sprite].
/// ### File format and access
/// File names must be formatted as ```NAME_WIDTH_HEIGHT.EXTENSION```.
/// Access keys into the pool are the full path to the image file (relative to the resource directory), followed by only NAME, width, height and extension are stripped.
/// This allows easy replacement of sprites with different formats.
/// ### Example
/// A file named ```mage_16_16.png``` in a subfolder ```/sprites/player``` of the resource folder will be accessible with the key ```/sprites/player/mage```.
pub struct SpritePool {
    /// The sprites currently stored in this pool for cloning.
    sprites: HashMap<String, Sprite>,
    /// The default-duration any newly loaded sprite will be initialized with. Mostly importat if you use references to sprites in this pool.
    default_duration: Duration,
}

impl SpritePool {
    /// Creates a new (empty) [SpritePool] instance.
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
            default_duration: Duration::ZERO,
        }
    }

    /// Sets a default duration this sprite pool will asign to every loaded sprite.
    /// This is especially useful when using a lot of Sprites only as references.
    pub fn with_default_duration(mut self, default_duration: Duration) -> Self {
        self.default_duration = default_duration;
        self
    }

    /// Loads all sprites within the given folder (relative to the good_web_game resource directory, see [good_web_game::context::ContextBuilder]) into the sprite pool.
    /// Can also search all subfolders.
    /// See [SpritePool] for required name formatting in order to load sprites correctly.
    pub fn with_path_list(
        mut self,
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
        path_list_path: impl AsRef<Path>,
        search_subfolders: bool,
    ) -> Self {
        let paths_file = ctx.filesystem.open(path_list_path.as_ref()).unwrap();
        let paths_reader = BufReader::new(paths_file);
        let paths = std::io::BufRead::lines(paths_reader);

        let sprite_match = regex::Regex::new(r"(.*)_\d*_\d*.[png|jpg|jpeg]").unwrap();

        for sub_path in paths.flatten() {
            let sub_path = sub_path.replace('\n', "");
            if sprite_match.is_match(&sub_path) {
                if let Ok(sprite) =
                    Sprite::from_path_fmt(sub_path.clone(), ctx, gfx_ctx, self.default_duration)
                {
                    self.sprites.insert(
                        sprite_match
                            .captures(&sub_path)
                            .map(|c| c.get(1).map(|m| m.as_str()))
                            .unwrap_or_default()
                            .unwrap_or_default()
                            .replace('\\', "/"),
                        sprite,
                    );
                }
            } else if search_subfolders {
                self = self.with_path_list(ctx, gfx_ctx, sub_path, search_subfolders);
            }
        }
        //println!("Now containing {} files.", self.sprites.len());
        self
    }

    /// Initialies a sprite from the sprite pool.
    /// The path syntax is exactly the same as for initalizing images or sprites, relative to the good_web_game resource folder.
    /// See [graphics::Image] and [Sprite].
    /// If the sprite (path) is not yet contained in the pool, an error is returned.
    /// For lazy initalization, use [SpritePool::init_sprite_lazy] instead.
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
    /// The path syntax is exactly the same as for initalizing images or sprites, relative to the good_web_game resource folder.
    /// See [graphics::Image] and [Sprite].
    /// If the sprite (path) is not yet contained in the pool, this panics.
    /// If you want to return an error, use [SpritePool::init_sprite] instead.
    /// For lazy initalization, use [SpritePool::init_sprite_lazy] instead.
    /// See [SpritePool] for rules related to key assignment.
    pub fn init_sprite_unchecked(&self, path: impl AsRef<Path>, frame_time: Duration) -> Sprite {
        let sprite = self
            .sprites
            .get(&path.as_ref().to_string_lossy().to_string())
            .unwrap_or_else(|| {
                panic!(
                    "[ERROR/Mooeye] Could not find sprite {}.",
                    path.as_ref().to_string_lossy()
                )
            });
        Sprite {
            frame_time,
            ..sprite.clone()
        }
    }

    // /// Initialies a sprite from the sprite pool.
    // /// The path syntax is exactly the same as for initalizing images or sprites, relative to the good_web_game resource folder.
    // /// See [graphics::Image] and [Sprite].
    // /// If the sprite (path) is not yet contained in the pool, the system will attempt to load it from the file system and return it.
    // /// If this also fails, an error is returned.
    // /// See [SpritePool] for rules related to key assignment.
    // pub fn init_sprite_lazy(
    //     &mut self,
    //     ctx: &mut Context,
    //     gfx_ctx: &mut GraphicsContext,
    //     path: impl AsRef<Path>,
    //     frame_time: Duration,
    // ) -> Result<Sprite, GameError> {
    //     // convert the key to a string
    //     let key = path.as_ref().to_string_lossy().to_string();
    //     // if it is not in the pool yet
    //     if !self.sprites.contains_key(&key) {
    //         // attempt to load the sprite
    //         self.attempt_load(ctx, gfx_ctx, &key)?;
    //     }
    //     // try to return the sprite (that should now be inserted).
    //     // If no sprite could be inserted, this will error
    //     self.init_sprite(path, frame_time)
    // }

    // /// Splits the given path into a folder and file name.
    // /// Then searches the folder for a file with the name and suffix _w_h.imageformat.
    // /// If found, loads that sprite into the pool.
    // fn attempt_load(
    //     &mut self,
    //     ctx: &mut Context,
    //     gfx_ctx: &mut GraphicsContext,
    //     key: &str,
    // ) -> Result<(), GameError> {
    //     // split off the directory to search
    //     let directory = key.rsplit_once('/').unwrap_or_default().0.to_owned() + "/";
    //     // get all branching paths
    //     let paths = ctx.fs.read_dir(directory)?;
    //     // genreate a regex to match image files
    //     let sprite_match = regex::Regex::new(r"(.*)_\d*_\d*.[png|jpg|jpeg]").unwrap();

    //     for sub_path in paths {
    //         // for every file in path
    //         let path_string = sub_path.to_string_lossy().to_string();
    //         // check if its an image
    //         if sprite_match.is_match(&path_string) {
    //             // check what name the sprite would have
    //             if let Some(Some(path_str)) = sprite_match
    //                 .captures(&path_string)
    //                 .map(|c| c.get(1).map(|m| m.as_str().replace('\\', "/").to_owned()))
    //             {
    //                 // compare to the requested name
    //                 if path_str == key {
    //                     // if it fits and can be loaded, put it into the pool
    //                     if let Ok(sprite) = Sprite::from_path_fmt(
    //                         sub_path.clone(),
    //                         ctx,
    //                         gfx_ctx,
    //                         self.default_duration,
    //                     ) {
    //                         self.sprites.insert(path_str, sprite);
    //                         return Ok(());
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     Err(GameError::CustomError(
    //         "[ERROR/Mooeye] Could not find sprite.".to_owned(),
    //     ))
    // }
}

impl Default for SpritePool {
    fn default() -> Self {
        Self::new()
    }
}
