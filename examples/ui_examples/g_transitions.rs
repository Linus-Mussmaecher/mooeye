use std::{
    collections::{HashMap},
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

pub struct GScene {
    gui: UiElement<()>,
}

impl GScene {
    pub fn new(ctx: &Context) -> GameResult<Self> {

        // At first, we create a general VBox to contain our UI
        let mut gui_box = containers::VerticalBox::new();

        // This title will change based on transitions whenever certain buttons are clicked.
        let title = Text::new("Move this element with the buttons.\nYou have not yet clicked a button.")
        // First, we style the title.
        .set_font("Alagard")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(0, ctx)
        // Then, we add a message handler to the element.
        .with_message_handler(

            // The message handle receives a set of messages and a mutable transition vector.
        |messages,_,transitions| {
            let ids = [11,12,13,21,22,23,];

            // We check if any of the buttons that interest us were clicked in the last frame.
            for id in ids{
                for message in messages{
                    if *message == UiMessage::<()>::Clicked(id){

                        // If yes, we add a new transition to the vector.
                        transitions.push_back(
                            // Transitions are initalized with the duration they should take to complete and augmented via the builder pattern.
                            Transition::new(Duration::ZERO)
                            // Here, we add a new content that will replace the old text once the transition completes.
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
        })
        .build();

        // Define a general visual style to use for all buttons.
        let vis = ui_element::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );

        // Create a grid box to hold all buttons.
        let mut grid_box = GridBox::new(2, 3);

        // Now, we create 6 buttons to move the element to all possible vertical and horizontal alignments and add them to the grid.
        let vert_up = Text::new(" ^ ")
            .set_font("Alagard")
            .to_owned()
            .to_element_builder(11, ctx)
            .with_visuals(vis) 
            .build();
        grid_box
            .add(vert_up, 0, 0)?;

        let vert_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned()
            .to_element_builder(12, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(vert_ce, 0, 1)?;

        let vert_do = Text::new(" v ")
            .set_font("Alagard")
            .to_owned().to_element_builder(13, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(vert_do, 0, 2)?;

        let hor_up = Text::new(" < ")
            .set_font("Alagard")
            .to_owned().to_element_builder(21, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_up, 1, 0)?;

        let hor_ce = Text::new(" . ")
            .set_font("Alagard")
            .to_owned().to_element_builder(22, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_ce, 1, 1)?;

        let hor_do = Text::new(" > ")
            .set_font("Alagard")
            .to_owned().to_element_builder(23, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_do, 1, 2)?;

        // The well-known back button will take us back to scene select.
        let back = Text::new("Back")
            .set_font("Alagard")
            .to_owned()
            .to_element_builder(31, ctx)
            .with_visuals(vis)
            .as_fill()
            .build();

        // We put the title, grid and back button together in a box.
        gui_box.add(title)?;
        gui_box.add(grid_box.to_element(30,ctx))?;
        gui_box.add(back)?;

        // Now we build the gui_box (note the technique of shadowing the variable)
        let gui_box = gui_box.to_element_builder(0, ctx)
        .with_size(ui_element::Size::Shrink(128., f32::INFINITY), ui_element::Size::Shrink(0., f32::INFINITY))
        .with_visuals(ui_element::Visuals::new(
            Color::from_rgb(120, 170, 200),
            Color::from_rgb(55, 67, 87),
            2.,
            0.,
        ))
        // Another message handler. Along with the messages and transition vector as discussed above, we also receive a layout struct
        // that represents the layout struct of the element in the moment the transition was initialized. This allows us to only change
        // the relevant parts of the layout without having to recreate it from scratch.
        .with_message_handler(|messages, layout, transitions| {
            // This guards prevent spam clicking a button from locking up the element with 1.5-second transitions.
            if !transitions.is_empty() {
                return;
            }
            let vert_map = HashMap::from([
                (11, ui_element::Alignment::Min),
                (12, ui_element::Alignment::Center),
                (13, ui_element::Alignment::Max),
            ]);
            let hor_map = HashMap::from([
                (21, ui_element::Alignment::Min),
                (22, ui_element::Alignment::Center),
                (23, ui_element::Alignment::Max),
            ]);
            // Check if any vertical buttons were clicked.
            for (key, val) in vert_map {
                if messages.contains(&UiMessage::Clicked(key)) {
                    transitions.push_back(
                        // If yes, add a transition.
                        Transition::new(Duration::from_secs_f32(1.5))
                        // This time, we don't change the content, but the layout.
                        // Layout, visuals and hover_visuals are not  changed on completion like content, but instead applied gradually.
                        .with_new_layout(ui_element::Layout{
                            y_alignment: val,
                            ..layout
                        }),
                    );
                }
            }
            // Repeat for horizontal keys.
            for (key, val) in hor_map {
                if messages.contains(&UiMessage::Clicked(key)) {
                    transitions.push_back(
                        Transition::new(Duration::from_secs_f32(1.5)).with_new_layout(ui_element::Layout{
                            x_alignment: val,
                            ..layout
                        }),
                    );
                }
            }
        })
        .build();

        Ok(Self { gui: gui_box })
    }
}


impl Scene for GScene {
    fn update(&mut self, ctx: &mut Context) -> Result<SceneSwitch, GameError> {
        // Nothing much to do here, except implement the back button functionality.
        // Note that the manage_messages functions will automatically exchange messages between the grid buttons and the title/box where they are handled by our message handlers.
        if self
            .gui
            .manage_messages(ctx, None)
            .contains(&UiMessage::Clicked(31))
        {
            return Ok(SceneSwitch::Pop(1));
        }

        Ok(SceneSwitch::None)
    
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {

        // Once again the basic drawing function.

        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        canvas.set_sampler(graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        canvas.finish(ctx)?;

        Ok(())
    }
}
