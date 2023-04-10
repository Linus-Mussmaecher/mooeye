use ggez::{*, graphics::Color};
use mooeye::{*, scene_manager::Scene};

use crate::{b_scene::BScene, c_uielement::CScene, g_transitions::GScene, d_containers::DScene};

pub struct SelectorScene{
    gui: UiElement<()>,
}

impl SelectorScene {

    pub fn new(ctx: &Context) -> Result<Self, GameError>{

        let vis = ui_element::Visuals {
            background: Color::from_rgb(49, 53, 69),
            border: Color::from_rgb(250, 246, 230),
            border_width: 2., rounded_corners: 4. 
        };

        let hover_vis = ui_element::Visuals {
            background: Color::from_rgb(83, 96, 150),
            border: Color::from_rgb(250, 246, 230),
            border_width: 2., rounded_corners: 4. 
        };

        let mut grid = containers::GridBox::new(3, 2);

        let contents = ["Scene", "UiElement", "Container", "Basic", "Messages", "Transition"];

        for i in 0..6{
            grid.add(
                graphics::Text::new(contents[i])
                .set_scale(32.)
                .to_owned()
                .to_element_builder(i as u32 + 1, ctx)
                .with_visuals(vis)
                .with_hover_visuals(hover_vis)
                .with_tooltip(
                    graphics::Text::new(format!("Click to look at the Scene created in the file {}.", contents[i].to_lowercase()))
                    .set_scale(24.)
                    .set_wrap(true)
                    .set_bounds(glam::Vec2::new(240.,500.))
                    .to_owned()
                    .to_element_builder(0, ctx)
                    .with_visuals(hover_vis)
                    .with_tooltip_layout()
                    .build()
                )
                .build()
                , i % 3, i / 3)?;
        }



        let grid = grid.to_element_builder(0, ctx)
        .build();

        Ok(Self { gui: grid })
    }
}

impl Scene for SelectorScene{
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&UiMessage::Clicked(1)){
            return Ok(scene_manager::SceneSwitch::push(BScene::new(ctx)));
        }
        
        if messages.contains(&UiMessage::Clicked(2)){
            return Ok(scene_manager::SceneSwitch::push(CScene::new(ctx)));
        }

        if messages.contains(&UiMessage::Clicked(3)){
            return Ok(scene_manager::SceneSwitch::push(DScene::new(ctx)?));
        }

        
        if messages.contains(&UiMessage::Clicked(6)){
            return Ok(scene_manager::SceneSwitch::push(GScene::new(ctx)?));
        }

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, Color::from_rgb(100, 100, 150));
        
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());



        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // The draw function should always end by finishing the canvas.
        canvas.finish(ctx)?;

        Ok(())
    }
}