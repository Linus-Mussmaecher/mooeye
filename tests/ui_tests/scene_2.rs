use std::collections::HashSet;

use mooeye::{scene_manager::{Scene, SceneSwitch}, ui_element::UiMessage};


use ggez::{
    context::Context,
    graphics::{Color, Text},
    *,
};
use mooeye::{UiElement, containers, UiContent};

pub struct Scene2 {
    gui: UiElement<()>,
}

impl Scene2 {
    pub fn new(ctx: &Context, score: i32) -> Self {
        let mut gui_box = containers::VerticalBox::new();

        let mut pi_img = graphics::Image::from_path(ctx, "/pi.png")
            .expect("Something went wrong loading /pi.png")
            .to_element_measured(2, &ctx);
        pi_img.layout.x_size = mooeye::ui_element::layout::Size::FIXED(96.);
        pi_img.layout.y_size = mooeye::ui_element::layout::Size::FIXED(96.);

        let score = Text::new(format!("Your score was {}. Yay!", score))
            .set_font("Alagard")
            .set_scale(28.)
            .to_owned()
            .to_element_measured(1, &ctx);

        let mut again = Text::new("Again")
            .set_font("Alagard")
            .set_scale(36.)
            .to_owned()
            .to_element_measured(3, &ctx);
        again.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
        );

        let mut quit = Text::new("Quit")
            .set_font("Alagard")
            .set_scale(36.)
            .to_owned()
            .to_element_measured(4, &ctx);

        quit.layout.x_size = mooeye::ui_element::layout::Size::FILL(64., f32::INFINITY);
        quit.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
        );

        let mut sub_box = containers::HorizontalBox::new();
        sub_box.add(again);
        sub_box.add(quit);

        let mut sub_box = sub_box.to_element(5);
        sub_box.layout.x_size = mooeye::ui_element::layout::Size::FILL(0., f32::INFINITY);

        gui_box.add(score);
        gui_box.add(pi_img);
        gui_box.add(sub_box);

        let mut gui_box = gui_box.to_element(0);

        gui_box.layout.x_size = mooeye::ui_element::layout::Size::SHRINK(128., f32::INFINITY);
        gui_box.layout.y_size = mooeye::ui_element::layout::Size::SHRINK(0., f32::INFINITY);
        gui_box.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(120, 170, 200),
            Color::from_rgb(55, 67, 87),
            2.,
        );

        Self { gui: gui_box }
    }
}

impl Scene for Scene2 {
    fn check_for_scene_switch(
        &mut self,
        ctx: &Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, GameError> {
        if self.gui.manage_messages(ctx, &HashSet::new()).contains(&UiMessage::Clicked(4)) {
            return Ok(SceneSwitch::Pop(2));
        }
        if self.gui.manage_messages(ctx, &HashSet::new()).contains(&UiMessage::Clicked(3)) {
            return Ok(SceneSwitch::Replace(
                2,
                Box::new(crate::scene_1::Scene1::new(&ctx)),
            ));
        }
        Ok(SceneSwitch::None)
    }
}

impl event::EventHandler<GameError> for Scene2 {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_rectangle(
            ctx,
            &mut canvas,
            graphics::Rect::new(0., 0., ctx.gfx.size().0, ctx.gfx.size().1),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
