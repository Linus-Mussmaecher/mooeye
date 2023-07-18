use ggez::{*, graphics::Color};
use mooeye::{ui, ui::UiContent, scene_manager};


// # UI, UI elements, UI element builder and basic messages
// In this example, our scene struct contains a gui consiting of a single UI element.


/// Another very basic scene. This time, it contains a UiElement called gui.
/// This is the root element of you GUI, and any interactions and acces to the GUI of this scene happen through this object.
pub struct CScene{
    /// The scenes GUI root element.
    gui: ui::UiElement<()>,
}

impl CScene {
    /// Creates a new CScene.
    /// Once again, we have no special parameters. Often, you ```new``` function will contain a lot of code
    /// basically describing the laoyut of you GUI, so this can get lengthy.
    /// Split in to helper functions as appropriate!    
    pub fn new(ctx: &Context) -> Self{
        // The scene constructor is usually the place where we create the state of our UI.
        // In this case, we will only create a single element.

        let text_element = 
        // First, we create anything implementing UiContent. ggez Image and Text do that, so we'll use a Text.
        // You can format that text as you can in ggez, so let's use our custom font and set a larger size.
        graphics::Text::new("Take me back!") 
        .set_font("Bahnschrift")
        .set_scale(32.)
        // Then we'll convert that content to an UiElementBuilder. We have to give it an ID.
        // ID 0 is reserved for elements not sending messages, but since we want to use the Text as a button, we'll use 1.
        .to_owned()
        .to_element_builder(1, ctx) 
        // We can now use the functions of UiElementBuilder to style and position our element.
        // First, we'll set the visuals using a Visuals struct.
        .with_visuals(ui::Visuals::new(
            Color::from_rgb(49, 53, 69),
            Color::from_rgb(250, 246, 230),
            4.,8. 
        ))
        // Additionally, you can add keycodes that make your element respond to key presses as it would respond to clicks
        .with_trigger_key(winit::event::VirtualKeyCode::A)
        // We can also set the alignment within the window...
        .with_alignment(ui::Alignment::Min, ui::Alignment::Center)
        // ... offset the element (note that we can pass None into most of these functions to leave the presets for one dimension untouched) ...
        .with_offset(25., None)
        // ... or set its padding ...
        .with_padding((5., 10., 5., 10.))
        // ... and size. Here, we use a special method 'shrink' that sets the size of both dimension to shrink without changing their boundaries.
        // Using the function .with_size would require us to also pass in boundaries.
        .as_shrink()
        // Finally, we build the element.
        .build();

        Self{gui: text_element}
    }

}

impl scene_manager::Scene for CScene{
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {

        // Usually, we would first perform our game logic here, but this scene has no logic.


        // You can get messages sent by your UI with the manage_messages function.
        // Usually, you also pass in extern messages created by your game state to bring the UI up to date. Since we don't have a game state, we can pass None (this is useful for menu scenes and similar).
        let messages = self.gui.manage_messages(ctx, None);

        // We then check if our button has been clicked by creating a Clicked event with the correct ID and checking if it is contained in the messages set.
        if messages.contains(&ui::UiMessage::Triggered(1)){
            // If it is, we end the current scene (and return to the previous one) by popping it off the stack.
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        // Otherwise, no scene switch is neccessary.
        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Once again, we first create a canvas and set a pixel sampler. Note that this time, we dont clear the background.
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);        
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());
        
        // Here, you would draw your gamestate.

        // Drawing a gui is as easy as calling draw_to_screen on the root element.
        // If you are using a scene, you can simply pass on the mouse_listen parameter. It will be managed by the scene manager.
        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);

        // Once again, we end drawing by finishing the canvas.
        canvas.finish(ctx)?;

        Ok(())
    }
}