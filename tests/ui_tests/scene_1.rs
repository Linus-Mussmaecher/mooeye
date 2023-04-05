use std::{collections::HashSet, time::Duration};

use ggez::{
    context::Context,
    graphics::{Color, Text},
    *,
};
use mooeye::{
    containers::{self, StackBox},
    sprite::Sprite,
    ui_element::{Alignment, Size},
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
            .to_element_measured(1, &ctx);

        let mut pi_img =
            //Sprite::from_path("/pi_sheet.png", ctx, 16, 24, Duration::from_secs_f32(0.2))
            Sprite::from_path_fmt("/pi_sheet_16_24.png", ctx, Duration::from_secs_f32(0.25))
                .expect("Spritesheet loading failed.")
                .to_element(2);

        pi_img.layout.x_size = pi_img.layout.x_size.scale(6.);
        pi_img.layout.y_size = pi_img.layout.y_size.scale(6.);
        pi_img.set_tooltip(Some({
            let mut text = Text::new("This is Pi!")
                .set_font("Alagard")
                .set_scale(28.)
                .to_owned()
                .to_element_measured(0, ctx);
            text.visuals = mooeye::ui_element::Visuals::new(
                Color::from_rgb(67, 99, 181),
                Color::from_rgb(45, 57, 77),
                2.,
                0.,
            );
            text.layout.x_alignment = Alignment::MIN;
            text.layout.y_alignment = Alignment::MIN;
            text.layout.x_size = text.layout.x_size.to_shrink();
            text.layout.y_size = text.layout.y_size.to_shrink();
            text
        }));

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
            2.,
            4.,
        ));

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

        let mut minipi = graphics::Image::from_path(ctx, "/pi.png")
            .expect("Something went wrong loading /pi.png")
            .to_element_measured(2, &ctx);
        minipi.layout.x_alignment = Alignment::MIN;
        minipi.layout.y_alignment = Alignment::MIN;
        minipi.layout.x_size = minipi.layout.x_size.to_shrink(); //TODO : THIS DOES NOT SHRINK
        minipi.layout.y_size = minipi.layout.y_size.to_shrink();
        minipi.layout.x_offset = -10.;
        minipi.layout.y_offset = -10.;

        let mut stack = StackBox::new();
        let playlayout = play.layout;
        stack.add(play);
        stack.add_top(minipi);
        let mut stack = stack.to_element(50);
        stack.layout = playlayout;
        stack.layout.x_size = Size::FILL(playlayout.x_size.min() + 10., f32::INFINITY);
        stack.layout.y_size = Size::FILL(playlayout.y_size.min() + 10., f32::INFINITY);
        stack.layout.padding = (0., 0., 0., 0.);

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
            2.,
            4.,
        ));

        gui_box.add(title);
        gui_box.add(pi_img);
        sub_box.add(stack);
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
