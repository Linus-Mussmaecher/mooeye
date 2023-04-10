use ggez::{*, graphics::Color};
use mooeye::{*, scene_manager::Scene, containers::StackBox};


// # Containers
// In this example, we learn about the 4 main types of containers provided with mooeye and use them to create a UI containing multiple elements.


pub struct DScene{
    gui: UiElement<()>,
}

impl DScene {

    
    pub fn new(ctx: &Context) -> Result<Self, GameError>{

        // Predefine some visuals so we don't have to do it for every element.

        let vis = ui_element::Visuals{
            background: Color::from_rgb(180, 120, 60),
            border: Color::from_rgb(18, 12, 6),
            border_width: 1.,
            rounded_corners: 0.,
        };

        let hover_vis = ui_element::Visuals{
            background: Color::from_rgb(160, 100, 40),
            border: Color::from_rgb(18, 12, 6),
            border_width: 3.,
            rounded_corners: 0.,
        };

        let cont_vis = ui_element::Visuals{
            background: Color::from_rgb(60, 120, 180),
            border: Color::from_rgb(180, 180, 190),
            border_width: 1.,
            rounded_corners: 0.,
        };
        
        // Note that the constructor now returns a Result. This is neccessary as the 'add' function used to add UI elements to containers can fail, thus failing the constructor.

        // The first container we use is a vertical box simply laying out elements from top to bottom.
        let mut ver_box = containers::VerticalBox::new();
        // We can manually change the spacing between elements in the box
        ver_box.spacing = 10.;
        // first, we need to add all the children to this vertical box.
        // We'll just use TextElement for now, but these can also be images, sprites, placeholders or more containers.
        for i in 0..8 {
            // Create an element.
            let element = graphics::Text::new(format!("{}", i))
            .set_font("Bahnschrift")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(vis)
            .build();
            // Add the element to the box. This can fail, if ver_box were not an actual container 
            // or a container that requires a special method for adding, like e.g. GridBox.
            ver_box.add(element)?;
        }
        // After adding all children, we can convert to a UiElement and style the box like we would style an other element. The usual pattern here is to shadow the variable to avoid use-after-move.
        let ver_box = ver_box
        .to_element_builder(0, ctx)
        .with_visuals(cont_vis)
        .build();

        // Another container we can use is GridBox. A GridBox needs to be initialized with a set height and width and cannot be extended.
        let mut grid = containers::GridBox::new(4, 4);
        // The contents of a grid box are initialized as empty elements.  We'll add buttons to the diagonal of the grid.
        for i in 0..4{
            // Create an element.
            let element = graphics::Text::new(format!("{}", i))
            .set_font("Bahnschrift")
            .set_scale(28.)
            .to_owned()
            .to_element_builder(0, ctx)
            .with_visuals(vis)
            // Elements can be given alignment and will align within their respecitive cell in the grid.
            .with_alignment(ui_element::Alignment::Max, None)
            .build();
            // Add the element to the box. This can fail, if ver_box were not an actual container 
            // or a container that requires a special method for adding, like e.g. GridBox.
            grid.add(element, i, i)?;
        }

        // We'll also create our usual back button and put it into the top right of the grid.

        let back = graphics::Text::new("Back!")
        .set_font("Bahnschrift")
        .set_scale(28.)
        .to_owned()
        .to_element_builder(1, ctx)
        .with_visuals(vis)
        .with_hover_visuals(hover_vis)
        .build();

        // This time, we'll enhance our back button a bit by using an icon that is displayed over the top left corner.
        // To achieve this, we'll use a StackBox.
        let mut stack = StackBox::new();
        stack.add(back)?;
        // The add_top function adds something to the top of a stack box. Creating and adding an element can be done inline.
        stack.add_top(graphics::Image::from_path(ctx, "/moo.png")?
        .to_element_builder(0, ctx)
        // We'll align the icon to the top right
        .with_alignment(ui_element::Alignment::Min,ui_element::Alignment::Min)
        // and offset it slightly
        .with_offset(-10., -10.)
        .build()
        )?;
        // to_element is a shorthand for to_element_builder().build() if we want to simply take the default builder and not change anything.
        let stack = stack.to_element(0, ctx);

        // Now, we add the stack to the grid.
        grid.add(
            stack,
            3, 0
        )?;

        // And finally build the grid.
        let grid = grid
        .to_element_builder(0, ctx)
        .with_visuals(cont_vis)
        .build();


        // The horizontal box is exactly the same as the vertical box except for orientation.
        // We will use a horizontal box to contain the boxes created so far.
        let mut hor_box = containers::HorizontalBox::new();
        hor_box.add(ver_box)?;
        hor_box.add(grid)?;
        let hor_box = hor_box.to_element(0, ctx);

        Ok(Self{gui: hor_box})
    }
}

impl  Scene for DScene {
    fn update(&mut self, ctx: &mut Context) -> Result<scene_manager::SceneSwitch, GameError> {
        // Nothing much to do here, except implement the back button functionality.

        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&UiMessage::Clicked(1)){
            // If it is, we end the current scene (and return to the previous one) by popping it off the stack.
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(&mut self, ctx: &mut Context, mouse_listen: bool) -> Result<(), GameError> {
        // Once again, we first create a canvas and set a pixel sampler. Note that this time, we dont clear the background.

        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);        
        canvas.set_sampler(ggez::graphics::Sampler::nearest_clamp());

        self.gui.draw_to_screen(ctx, &mut canvas, mouse_listen);
        
        canvas.finish(ctx)?;

        Ok(())
    }
}

