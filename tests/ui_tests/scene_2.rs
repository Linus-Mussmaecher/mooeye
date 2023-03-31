use std::{collections::{HashSet, HashMap}, time::Duration};

use mooeye::{
    containers::{HorizontalBox, VerticalBox},
    scene_manager::{Scene, SceneSwitch},
    ui_element::{UiMessage, layout::Alignment, Transition},
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
        pi_img.layout.x_size = mooeye::ui_element::layout::Size::FIXED(96.);
        pi_img.layout.y_size = mooeye::ui_element::layout::Size::FIXED(96.);

        let title = Text::new(format!(
            "Move this element with the buttons.\nHere is a number: {}.",
            score
        ))
        .set_font("Alagard")
        .set_scale(28.)
        .to_owned()
        .to_element_measured(1, &ctx);

        let vis = mooeye::ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );

        let mut vert_box = VerticalBox::new();
        let mut vert_up = Text::new(" ^ ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(11, ctx);
        vert_up.visuals = vis;
        vert_box.add(vert_up);
        let mut vert_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(12, ctx);
        vert_ce.visuals = vis;
        vert_box.add(vert_ce);
        let mut vert_do = Text::new( " v ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(13, ctx);
        vert_do.visuals = vis;
        vert_box.add(vert_do);

        let mut hor_box = VerticalBox::new();
        let mut hor_up = Text::new(" < ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(21, ctx);
        hor_up.visuals = vis;
        hor_box.add(hor_up);
        let mut hor_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(22, ctx);
        hor_ce.visuals = vis;
        hor_box.add(hor_ce);
        let mut hor_do = Text::new(" > ")
            .set_font("Alagard")
            .to_owned()
            .to_element_measured(23, ctx);
        hor_do.visuals = vis;
        hor_box.add(hor_do);

        let mut sub_box = HorizontalBox::new();
        sub_box.add(vert_box.to_element(10));
        sub_box.add(hor_box.to_element(20));

        gui_box.add(title);
        gui_box.add(pi_img);
        gui_box.add(sub_box.to_element(30));

        let mut gui_box = gui_box.to_element(0);

        gui_box.layout.x_size = mooeye::ui_element::layout::Size::SHRINK(128., f32::INFINITY);
        gui_box.layout.y_size = mooeye::ui_element::layout::Size::SHRINK(0., f32::INFINITY);
        gui_box.visuals = mooeye::ui_element::Visuals::new(
            Color::from_rgb(120, 170, 200),
            Color::from_rgb(55, 67, 87),
            2.,
            0.,
        );

        gui_box.set_message_handler(|messages, layout, transitions| {
            if !transitions.is_empty(){
                return;
            }
            let vert_map = HashMap::from([
                (11, Alignment::MIN),
                (12, Alignment::CENTER),
                (13, Alignment::MAX)
            ]);
            let hor_map = HashMap::from([
                (21, Alignment::MIN),
                (22, Alignment::CENTER),
                (23, Alignment::MAX)
            ]);
            for (key, val) in vert_map{
                if messages.contains(&UiMessage::Clicked(key)){
                    transitions.push_back(Transition::new(Duration::from_secs_f32(1.5)).with_new_layout(
                        {
                            let mut new_layout = layout;
                            new_layout.y_alignment = val;
                            new_layout
                        }
                    ));
                }
            }
            for (key, val) in hor_map{
                if messages.contains(&UiMessage::Clicked(key)){
                    transitions.push_back(Transition::new(Duration::from_secs_f32(1.5)).with_new_layout(
                        {
                            let mut new_layout = layout;
                            new_layout.x_alignment = val;
                            new_layout
                        }
                    ));
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
            .contains(&UiMessage::Clicked(4))
        {
            return Ok(SceneSwitch::Pop(2));
        }
        if self
            .gui
            .manage_messages(ctx, &HashSet::new())
            .contains(&UiMessage::Clicked(3))
        {
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
            graphics::Rect::new(0., 0., 
                ctx.gfx.window().inner_size().width as f32,
                ctx.gfx.window().inner_size().height as f32,),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
