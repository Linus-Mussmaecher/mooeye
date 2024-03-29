use std::{
    collections::HashMap,
    time::Duration,
};

use mooeye::{ui, ui::UiContent, scene_manager};

use ggez::{
    context::Context,
    graphics::{Color, Text},
    *,
};

/// A once again basic scene containing only a GUI.
pub struct EScene {
    /// The root element of EScene's GUI.
    gui: ui::UiElement<()>,
}

impl EScene {
    /// Creates a new 'default' EScene.
    pub fn new(ctx: &Context) -> GameResult<Self> {


        // This title will change based on transitions whenever certain buttons are clicked.
        let title = Text::new("Move this element with the buttons.\nYou have not yet clicked a button.")
        // First, we style the title.
        .set_font("Bahnschrift")
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
                    if *message == ui::UiMessage::<()>::Clicked(id){

                        // If yes, we add a new transition to the vector.
                        transitions.push_back(
                            // Transitions are initalized with the duration they should take to complete and augmented via the builder pattern.
                            ui::Transition::new(Duration::ZERO)
                            // Here, we add a new content that will replace the old text once the transition completes.
                            .with_new_content(Text::new(format!(
                                "Move this element with the buttons.\nYou clicked a button with id {}.",
                                id
                            ))
                            .set_font("Bahnschrift")
                            .set_scale(24.)
                            .to_owned())
                        )
                        
                    }
                }
            }
        })
        .build();

        // Define a general visual style to use for all buttons.
        let vis = ui::Visuals::new(
            Color::from_rgb(77, 109, 191),
            Color::from_rgb(55, 67, 87),
            2.,
            4.,
        );

        // Create a grid box to hold all buttons.
        let mut grid_box = ui::containers::GridBox::new(2, 3);

        // Now, we create 6 buttons to move the element to all possible vertical and horizontal alignments and add them to the grid.
        let vert_up = Text::new(" ^ ")
            .set_font("Bahnschrift")
            .to_owned()
            .to_element_builder(11, ctx)
            .with_visuals(vis) 
            // We can also set a sound to be played on click/key press
            .with_trigger_sound(ggez::audio::Source::new(ctx, "/blipSelect.wav").ok())
            .build();
        grid_box
            .add(vert_up, 0, 0)?;

        let vert_ce = Text::new(" . ")
            .set_font("Bahnschrift")
            .to_owned()
            .to_element_builder(12, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(vert_ce, 0, 1)?;

        let vert_do = Text::new(" v ")
            .set_font("Bahnschrift")
            .to_owned().to_element_builder(13, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(vert_do, 0, 2)?;

        let hor_up = Text::new(" < ")
            .set_font("Bahnschrift")
            .to_owned().to_element_builder(21, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_up, 1, 0)?;

        let hor_ce = Text::new(" . ")
            .set_font("Bahnschrift")
            .to_owned().to_element_builder(22, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_ce, 1, 1)?;

        let hor_do = Text::new(" > ")
            .set_font("Bahnschrift")
            .to_owned().to_element_builder(23, ctx)
            .with_visuals(vis)
            .build();
        grid_box
            .add(hor_do, 1, 2)?;

        // The well-known back button will take us back to scene select.
        let back = Text::new("Back")
            .set_font("Bahnschrift")
            .to_owned()
            .to_element_builder(1, ctx)
            .with_visuals(vis)
            .as_fill()
            .build();

        
        // We create a general VBox to contain our UI
        // We can create Vertical and Horizontal Boxes with the 'spaced' constructor to set its spacing value.
        let gui_box = ui::containers::VerticalBox::new_spaced(6.)
        .to_element_builder(0, ctx)
        // We put the title, grid and back button together in a box.
        .with_child(title)
        .with_child(grid_box.to_element(30, ctx))
        .with_child(back)
        .with_size(ui::Size::Shrink(128., f32::INFINITY), ui::Size::Shrink(0., f32::INFINITY))
        .with_visuals(ui::Visuals::new(
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
                (11, ui::Alignment::Min),
                (12, ui::Alignment::Center),
                (13, ui::Alignment::Max),
            ]);
            let hor_map = HashMap::from([
                (21, ui::Alignment::Min),
                (22, ui::Alignment::Center),
                (23, ui::Alignment::Max),
            ]);
            // Check if any vertical buttons were clicked.
            for (key, val) in vert_map {
                if messages.contains(&ui::UiMessage::Triggered(key)) {
                    transitions.push_back(
                        // If yes, add a transition.
                        ui::Transition::new(Duration::from_secs_f32(1.5))
                        // This time, we don't change the content, but the layout.
                        // Layout, visuals and hover_visuals are not  changed on completion like content, but instead applied gradually.
                        .with_new_layout(ui::Layout{
                            y_alignment: val,
                            ..layout
                        }),
                    );
                }
            }
            // Repeat for horizontal keys.
            for (key, val) in hor_map {
                if messages.contains(&ui::UiMessage::Triggered(key)) {
                    transitions.push_back(
                        ui::Transition::new(Duration::from_secs_f32(1.5)).with_new_layout(ui::Layout{
                            x_alignment: val,
                            ..layout
                        }),
                    );
                }
            }
        })
        .build();

        // Finally, we wrap our gui_box into a space-filling stack pane so we have a place to later add further elements

        Ok(Self {
            gui: ui::containers::StackBox::new()
            .to_element_builder(100, ctx)
            .as_fill()
            .with_child(gui_box)
            .build()
        })
    }
}


impl scene_manager::Scene for EScene {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // Nothing much to do here, except implement the back button functionality.

        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)){
            // If it is, we end the current scene (and return to the previous one) by popping it off the stack.
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        if messages.contains(&ui::UiMessage::Triggered(13)){
            // If a certain button is pressed, add a small text element to the gui.
            self.gui.add_element(100,
                // using a duration box as a wrapper will remove the element after a set amount of time
                  ui::containers::DurationBox::new(
                    Duration::from_secs_f32(1.5),
                     graphics::Text::new("Just a small reminder that you pressed button 13.")
                     .set_font("Bahnschrift")
                     .set_wrap(true)
                     .set_bounds(glam::Vec2::new(200., 500.))
                     .set_scale(28.)
                     .to_owned()
                     .to_element_builder(0, ctx)
                     .with_visuals(ui::Visuals::new(
                        Color::from_rgb(77, 109, 191),
                        Color::from_rgb(55, 67, 87),
                        2.,
                        4.,
                    ))
                     .build()
                    ).to_element_builder(0, ctx)
                    .with_alignment(ui::Alignment::Center, ui::Alignment::Min)
                    .with_offset(0., 25.)
                    .build()
                    );
        }

        Ok(scene_manager::SceneSwitch::None)
    
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