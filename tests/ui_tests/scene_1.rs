use std::{collections::HashSet, time::Duration};

use ggez::{
    context::Context,
    graphics::{Color, Text},
    *,
};
use mooeye::{
    containers::{self, StackBox},
    sprite::Sprite,
    ui_element::{Alignment, UiElementBuilder},
    UiContent, UiElement,
};
use mooeye::{
    scene_manager::{Scene, SceneSwitch},
    ui_element::{Transition, UiMessage},
};

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
            .to_element_builder(1, ctx)
            .build();

        let pi_img =
            Sprite::from_path_fmt("/pi_sheet_16_24.png", ctx, Duration::from_secs_f32(0.25))
                .expect("Spritesheet loading failed.")
                .to_element_builder(0, ctx)
                .scaled(6., 6.)
                .with_tooltip(
                    UiElementBuilder::new(
                        0,
                        Text::new("This is Pi!")
                            .set_font("Alagard")
                            .set_scale(28.)
                            .to_owned(),
                    )
                    .with_tooltip_layout()
                    .build(),
                )
                .build();

        let mut sub_box = containers::VerticalBox::new();

        let mut play = Text::new("Play")
            .set_font("Alagard")
            .set_scale(36.)
            .to_owned()
            .to_element_builder(3, ctx)
            .with_visuals(mooeye::ui_element::Visuals::new(
                Color::from_rgb(77, 109, 191),
                Color::from_rgb(55, 67, 87),
                2.,
                4.,
            ))
            .with_hover_visuals(mooeye::ui_element::Visuals::new(
                Color::from_rgb(67, 89, 201),
                Color::from_rgb(65, 77, 107),
                2.,
                4.,
            ))
            .build();

        play.add_transition(Transition::new(Duration::from_secs(1)));
        play.add_transition(
            Transition::new(Duration::from_secs(2))
                .with_new_visuals(mooeye::ui_element::Visuals::new(
                    Color::from_rgb(191, 89, 81),
                    Color::from_rgb(55, 67, 87),
                    2.,
                    4.,
                ))
                .with_new_content(
                    Text::new("Start")
                        .set_font("Alagard")
                        .set_scale(36.)
                        .to_owned(),
                ),
        );

        let minipi = graphics::Image::from_path(ctx, "/pi.png")
            .expect("Something went wrong loading /pi.png")
            .to_element_builder(3, ctx)
            .with_alignment(Alignment::MIN, Alignment::MIN)
            .as_shrink()
            .with_offset(-10., -10.)
            .build();

        let mut stack = StackBox::new();
        let playlayout = play.get_layout();
        stack.add(play);
        stack.add_top(minipi);
        let stack = UiElementBuilder::new(0, stack)
            .with_wrapper_layout(playlayout)
            .build();

        let quit = Text::new("Quit")
            .set_font("Alagard")
            .set_scale(36.)
            .to_owned()
            .to_element_builder(4, ctx)
            .with_visuals(mooeye::ui_element::Visuals::new(
                Color::from_rgb(77, 109, 191),
                Color::from_rgb(55, 67, 87),
                2.,
                4.,
            ))
            .with_hover_visuals(mooeye::ui_element::Visuals::new(
                Color::from_rgb(67, 89, 201),
                Color::from_rgb(65, 77, 107),
                2.,
                4.,
            ))
            .build();

        gui_box.add(title);
        gui_box.add(pi_img);
        sub_box.add(stack);
        sub_box.add(quit);
        gui_box.add(sub_box.to_element(0, ctx));

        let gui_box = gui_box
            .to_element_builder(0, ctx)
            .with_visuals(mooeye::ui_element::Visuals::new(
                Color::from_rgb(120, 170, 200),
                Color::from_rgb(55, 67, 87),
                2.,
                10.,
            ))
            .build();

        Self { gui: gui_box }
    }
}

impl Scene for Scene1 {
    fn update(&mut self, ctx: &mut Context) -> Result<SceneSwitch, GameError> {
        if self
            .gui
            .manage_messages(ctx, &HashSet::new())
            .contains(&UiMessage::Clicked(4))
        {
            return Ok(SceneSwitch::Pop(1));
        }
        if self
            .gui
            .manage_messages(ctx, &HashSet::new())
            .contains(&UiMessage::Clicked(3))
        {
            return Ok(SceneSwitch::Push(Box::new(crate::scene_2::Scene2::new(
                ctx, 35,
            ))));
        }
        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 100, 150));
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;
        Ok(())
    }
}
