use std::{collections::HashSet, time::Duration};


use ggez::{
    context::Context,
    graphics::{Color, Text},
    *,
};
use mooeye::{scene_manager::{Scene, SceneSwitch}, ui_element::{UiMessage, Transition}};
use mooeye::{containers, UiContent, UiElement};

pub struct Scene1 {
    gui: UiElement<u32>,
}


impl Scene1 {
    pub fn new(ctx: &Context) -> Self {
        let mut gui_box = containers::VerticalBox::new();

        let title = Text::new("Pideo Game")
            .set_font("Alagard")
            .set_scale(54.)
            .to_owned()
            .to_element_measured(1, &ctx);

        let mut pi_img = graphics::Image::from_path(ctx, "/pi.png")
            .expect("Error when loading file /pi.png")
            .to_element_measured(2, &ctx);
        pi_img.layout.x_size = pi_img.layout.x_size.scale(6.);
        pi_img.layout.y_size = pi_img.layout.y_size.scale(6.);

        let mut sub_box = containers::VerticalBox::new();

        let mut play = Text::new("Play")
            .set_font("Alagard")
            .set_scale(36.)
            .to_owned()
            .to_element_measured(3, &ctx);

        play.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );
        play.hover_visuals = Some(mooeye::ui_element::Visuals::new(
            Color::from_rgb(67, 89, 201),
            Color::from_rgb(65, 77, 107),
            5.,
            10.
        ));
        play.add_transition(Transition::new(Duration::from_secs(10))
            .with_new_visuals(mooeye::ui_element::Visuals::new(
                Color::from_rgb(191, 89, 81),
                Color::from_rgb(55, 67, 87),
                0.,
                4.,
            ))
        );

        let mut quit = Text::new("Quit")
            .set_font("Alagard")
            .set_scale(36.)
            .to_owned()
            .to_element_measured(4, &ctx);

        quit.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );
        quit.hover_visuals = Some(mooeye::ui_element::Visuals::new(
            Color::from_rgb(67, 89, 201),
            Color::from_rgb(65, 77, 107),
            5.,
            10.,
        ));

        gui_box.add(title);
        gui_box.add(pi_img);
        sub_box.add(play);
        sub_box.add(quit);
        gui_box.add(sub_box.to_element(5));

        let mut gui_box = gui_box.to_element(0);
        gui_box.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(120, 170, 200),
            Color::from_rgb(55, 67, 87),
            2.,
            10.,
        );

        Self { gui: gui_box }
    }
}

impl Scene for Scene1 {
    fn check_for_scene_switch(
        &mut self,
        ctx: &Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, GameError> {
        if self.gui.manage_messages(ctx, &HashSet::new()).contains(&UiMessage::Clicked(4)) {
            return Ok(SceneSwitch::Pop(1));
        }
        if self.gui.manage_messages(ctx, &HashSet::new()).contains(&UiMessage::Clicked(3)) {
            return Ok(SceneSwitch::Replace(
                1,
                Box::new(crate::scene_2::Scene2::new(ctx, 35)),
            ));
        }
        Ok(SceneSwitch::None)
    }
}

impl event::EventHandler<GameError> for Scene1 {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 100, 150));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());
        self.gui.draw_to_rectangle(
            ctx,
            &mut canvas,
            ggez::graphics::Rect::new(
                0.,
                0.,
                ctx.gfx.window().inner_size().width as f32,
                ctx.gfx.window().inner_size().height as f32,
            ),
        );

        canvas.finish(ctx)?;
        Ok(())
    }
}
