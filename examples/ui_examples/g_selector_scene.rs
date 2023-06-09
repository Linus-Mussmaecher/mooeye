use ggez::{graphics::Color, *};
use mooeye::{scene_manager::Scene, *};

/// This scene is the main-drop in scene and allows you to access the different tutorial scenes.
/// This is not intended to be a tutorial in itself, but feel free to read it to get a feeling for mooeye.
pub struct SelectorScene {
    gui: UiElement<()>,
}

impl SelectorScene {
    pub fn new(ctx: &Context) -> Result<Self, GameError> {
        // Defining visuals

        let vis = ui_element::Visuals::new(
            Color::from_rgb(180, 120, 60),
            Color::from_rgb(18, 12, 6),
            1.,
            0.,
        );

        let hover_vis = ui_element::Visuals::new(
            Color::from_rgb(160, 100, 40),
            Color::from_rgb(18, 12, 6),
            3.,
            0.,
        );

        // Creating main grid

        let mut grid = containers::GridBox::new(3, 2);

        let contents = ["Scene", "UiElement", "Container", "Messages", "Sprites"];

        for (i, &cont) in contents.iter().enumerate() {
            grid.add(
                graphics::Text::new(cont)
                    .set_scale(32.)
                    .to_owned()
                    .to_element_builder(i as u32 + 1, ctx)
                    .with_visuals(vis)
                    .with_hover_visuals(hover_vis)
                    .with_tooltip(
                        graphics::Text::new(format!(
                            "Click to look at the Scene created in the file {}.",
                            contents[i].to_lowercase()
                        ))
                        .set_scale(24.)
                        .set_wrap(true)
                        .set_bounds(glam::Vec2::new(240., 500.))
                        .to_owned()
                        .to_element_builder(0, ctx)
                        .with_visuals(hover_vis)
                        .build(),
                    )
                    .build(),
                i % 3,
                i / 3,
            )?;
        }

        // Add quit button

        grid.add(
            graphics::Text::new("Quit")
                .set_scale(32.)
                .to_owned()
                .to_element_builder(6, ctx)
                .with_visuals(vis)
                .with_hover_visuals(hover_vis)
                .build(),
            2,
            1,
        )?;

        Ok(Self {
            gui: grid.to_element_builder(0, ctx).build(),
        })
    }
}

impl Scene for SelectorScene {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        // Scene switches for different scenes

        if messages.contains(&UiMessage::Triggered(1)) {
            return Ok(scene_manager::SceneSwitch::push(
                crate::b_scene::BScene::new(ctx),
            ));
        }

        if messages.contains(&UiMessage::Triggered(2)) {
            return Ok(scene_manager::SceneSwitch::push(
                crate::c_uielement::CScene::new(ctx),
            ));
        }

        if messages.contains(&UiMessage::Triggered(3)) {
            return Ok(scene_manager::SceneSwitch::push(
                crate::d_containers::DScene::new(ctx)?,
            ));
        }

        if messages.contains(&UiMessage::Triggered(4)) {
            return Ok(scene_manager::SceneSwitch::push(
                crate::e_messages::EScene::new(ctx)?,
            ));
        }

        if messages.contains(&UiMessage::Triggered(5)) {
            return Ok(scene_manager::SceneSwitch::push(
                crate::f_sprites::FScene::new(ctx)?,
            ));
        }

        // Exit

        if messages.contains(&UiMessage::Triggered(6)) {
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // business as usual
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 100, 150));

        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;

        Ok(())
    }
}
