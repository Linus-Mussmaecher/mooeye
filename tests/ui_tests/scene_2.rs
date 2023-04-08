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

        let pi_img = graphics::Image::from_path(ctx, "/pi.png")
            .expect("Something went wrong loading /pi.png")
            .to_element_builder(2, ctx).with_size(ui_element::Size::FIXED(96.), ui_element::Size::FIXED(96.)).build();

        let title = Text::new(format!(
            "Move this element with the buttons.\nHere is a number: {}.",
            score
        ))
        .set_font("Alagard")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(0, ctx).with_message_handler(
        |messages,_,transitions| {
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
        }).build();

        let vis = ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );

        let mut grid_box = GridBox::new(2, 3);

        let vert_up = Text::new(" ^ ")
            .set_font("Alagard")
            .to_owned()
            .to_element_builder(11, ctx)
            .with_visuals(vis) 
            .build();
        grid_box
            .add(vert_up, 0, 0)
            .expect("Index Out Of Bounds, probably.");
        let vert_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned()
            .to_element_builder(12, ctx)
            .with_visuals(vis)
            .with_tooltip(
                Text::new("Move the element to the vertical center of the screen.")
                    .set_font("Alagard")
                    .set_wrap(true)
                    .set_bounds(glam::Vec2::new(200., 500.))
                    .set_scale(20.)
                    .to_owned()
                    .to_element_builder(0, ctx)
                    .with_visuals(vis)
                    .with_tooltip_layout()
                    .build()
            )
            .build();
        grid_box
            .add(vert_ce, 0, 1)
            .expect("Index Out Of Bounds, probably.");
        let vert_do = Text::new(" v ")
            .set_font("Alagard")
            .to_owned().to_element_builder(13, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(vert_do, 0, 2)
            .expect("Index Out Of Bounds, probably.");

        //let mut hor_box = VerticalBox::new();
        let hor_up = Text::new(" < ")
            .set_font("Alagard")
            .to_owned().to_element_builder(21, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_up, 1, 0)
            .expect("Index Out Of Bounds, probably.");
        let hor_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned().to_element_builder(22, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_ce, 1, 1)
            .expect("Index Out Of Bounds, probably.");
        let hor_do = Text::new(" > ")
            .set_font("Alagard")
            .to_owned().to_element_builder(23, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_do, 1, 2)
            .expect("Index Out Of Bounds, probably.");

        let back = Text::new("Back")
            .set_font("Alagard")
            .to_owned()
            .to_element_builder(31, ctx)
            .with_visuals(vis)
            .as_fill()
            .build();

        gui_box.add(title);
        gui_box.add(pi_img);
        gui_box.add(grid_box.to_element(30,ctx));
        gui_box.add(back);

        let gui_box = gui_box.to_element_builder(0, ctx)
        .with_size(ui_element::Size::SHRINK(128., f32::INFINITY), ui_element::Size::SHRINK(0., f32::INFINITY))
        .with_visuals(ui_element::Visuals::new(
            Color::from_rgb(120, 170, 200),
            Color::from_rgb(55, 67, 87),
            2.,
            0.,
        ))
        .with_message_handler(|messages, layout, transitions| {
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
        })
        .build();

        Self { gui: gui_box }
    }
}


impl Scene for Scene2 {
    fn update(&mut self, ctx: &mut Context) -> Result<SceneSwitch, GameError> {
        if self
            .gui
            .manage_messages(ctx, &HashSet::new())
            .contains(&UiMessage::Clicked(31))
        {
            return Ok(SceneSwitch::Pop(1));
        }
        Ok(SceneSwitch::None)
    
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;

        Ok(())
    }
}
