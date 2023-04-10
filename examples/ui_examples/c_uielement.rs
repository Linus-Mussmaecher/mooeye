use ggez::{*, graphics::Color};
use mooeye::{*, scene_manager::Scene};


// # UI, UI elements, UI element builder and basic messages
// In this example, our scene struct contains a gui consiting of a single UI element.


pub struct CScene{
    gui: UiElement<()>,
}

impl CScene {

    
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
        .with_visuals(ui_element::Visuals {
            background: Color::from_rgb(49, 53, 69),
            border: Color::from_rgb(250, 246, 230),
            border_width: 4., rounded_corners: 8. 
        })
        // We can also set the alignment within the window...
        .with_alignment(ui_element::Alignment::Min, ui_element::Alignment::Center)
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

impl Scene for CScene{
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {

        // Usually, we would first perform our game logic here, but this scene has no logic.


        // You can get messages sent by your UI with the manage_messages function.
        // Usually, you also pass in extern messages created by your game state to bring the UI up to date. Since we don't have a game state, we can pass None (this is useful for menu scenes and similar).
        let messages = self.gui.manage_messages(ctx, None);

        // We then check if our button has been clicked by creating a Clicked event with the correct ID and checking if it is contained in the messages set.
        if messages.contains(&UiMessage::Clicked(1)){
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