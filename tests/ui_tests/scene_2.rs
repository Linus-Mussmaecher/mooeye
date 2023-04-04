use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use mooeye::{
    containers::grid_box::GridBox,
    scene_manager::{Scene, SceneSwitch},
    ui_element::Transition,
    UiMessage, *,
};

use ggez::{
    context::Context,
    graphics::{Color, Text},
    *,
};
use mooeye::{containers, UiContent, UiElement};

pub struct Scene2 {
    gui: UiElement<()>,
}

impl Scene2 {
    pub fn new(ctx: &Context, score: i32) -> Self {
        let mut gui_box = containers::VerticalBox::new();

        let mut pi_img = graphics::Image::from_path(ctx, "/pi.png")
            .expect("Something went wrong loading /pi.png")
            .to_element_measured(2, &ctx);
        pi_img.layout.x_size = ui_element::Size::FIXED(96.);
        pi_img.layout.y_size = ui_element::Size::FIXED(96.);

        let mut title = Text::new(format!(
            "Move this element with the buttons.\nHere is a number: {}.",
            score
        ))
        .set_font("Alagard")
        .set_scale(28.)
        .to_owned()
        .to_element_measured(1, &ctx);
        title.set_message_handler(|messages,_,transitions| {
            let ids = [11,12,13,21,22,23,];
            for id in ids{
                for message in messages{
                    if *message == UiMessage::<()>::Clicked(id){
                        transitions.push_back(
                            Transition::new(Duration::ZERO)
                            .with_new_content(Text::new(format!(
                                "Move this element with the buttons.\nYou clicked a button with id {}.",
                                id
                            ))
                            .set_font("Alagard")
                            .set_scale(24.)
                            .to_owned())
                        )
                        
                    }
                }
            }
        });

        let vis = ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );

        let mut grid_box = GridBox::new(2, 3);

        let mut vert_up = Text::new(" ^ ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(11, ctx);
        vert_up.visuals = vis;
        grid_box
            .add(vert_up, 0, 0)
            .expect("Index Out Of Bounds, probably.");
        let mut vert_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(12, ctx);
        vert_ce.visuals = vis;
        vert_ce.set_tooltip(Some(
            ui_element::make_tooltip(Text::new("Move the element to the vertical center of the screen.")
            .set_font("Alagard")
            .set_wrap(true)
            .set_bounds(glam::Vec2::new(200., 500.))
            .set_scale(20.)
            .to_owned()
            .to_element_measured(12, ctx), vis)
        ));
        grid_box
            .add(vert_ce, 0, 1)
            .expect("Index Out Of Bounds, probably.");
        let mut vert_do = Text::new(" v ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(13, ctx);
        vert_do.visuals = vis;
        grid_box
            .add(vert_do, 0, 2)
            .expect("Index Out Of Bounds, probably.");

        //let mut hor_box = VerticalBox::new();
        let mut hor_up = Text::new(" < ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(21, ctx);
        hor_up.visuals = vis;
        grid_box
            .add(hor_up, 1, 0)
            .expect("Index Out Of Bounds, probably.");
        let mut hor_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(22, ctx);
        hor_ce.visuals = vis;
        grid_box
            .add(hor_ce, 1, 1)
            .expect("Index Out Of Bounds, probably.");
        let mut hor_do = Text::new(" > ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(23, ctx);
        hor_do.visuals = vis;
        grid_box
            .add(hor_do, 1, 2)
            .expect("Index Out Of Bounds, probably.");

        let mut back = Text::new("Back")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(31, ctx);
        back.visuals = vis;
        back.layout.y_size =
            ui_element::Size::FILL(back.layout.y_size.min(), f32::INFINITY);

        gui_box.add(title);
        gui_box.add(pi_img);
        gui_box.add(grid_box.to_element(30));
        gui_box.add(back);

        let mut gui_box = gui_box.to_element(0);

        gui_box.layout.x_size = ui_element::Size::SHRINK(128., f32::INFINITY);
        gui_box.layout.y_size = ui_element::Size::SHRINK(0., f32::INFINITY);
        gui_box.visuals = ui_element::Visuals::new(
            Color::from_rgb(120, 170, 200),
            Color::from_rgb(55, 67, 87),
            2.,
            0.,
        );

        gui_box.set_message_handler(|messages, layout, transitions| {
            if !transitions.is_empty() {
                return;
            }
            let vert_map = HashMap::from([
                (11, ui_element::Alignment::MIN),
                (12, ui_element::Alignment::CENTER),
                (13, ui_element::Alignment::MAX),
            ]);
            let hor_map = HashMap::from([
                (21, ui_element::Alignment::MIN),
                (22, ui_element::Alignment::CENTER),
                (23, ui_element::Alignment::MAX),
            ]);
            for (key, val) in vert_map {
                if messages.contains(&UiMessage::Clicked(key)) {
                    transitions.push_back(
                        Transition::new(Duration::from_secs_f32(1.5)).with_new_layout({
                            let mut new_layout = layout;
                            new_layout.y_alignment = val;
                            new_layout
                        }),
                    );
                }
            }
            for (key, val) in hor_map {
                if messages.contains(&UiMessage::Clicked(key)) {
                    transitions.push_back(
                        Transition::new(Duration::from_secs_f32(1.5)).with_new_layout({
                            let mut new_layout = layout;
                            new_layout.x_alignment = val;
                            new_layout
                        }),
                    );
                }
            }
        });

        Self { gui: gui_box }
    }
}

impl Scene for Scene2 {
    fn check_for_scene_switch(
        &mut self,
        ctx: &Context,
    ) -> Result<mooeye::scene_manager::SceneSwitch, GameError> {
        if self
            .gui
            .manage_messages(ctx, &HashSet::new())
            .contains(&UiMessage::Clicked(31))
        {
            return Ok(SceneSwitch::Pop(1));
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

        self.gui.draw_to_screen(ctx, &mut canvas);

        canvas.finish(ctx)?;

        Ok(())
    }
}
