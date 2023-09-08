use good_web_game::{event::GraphicsContext, graphics::Color, *};
use mooeye::{scene_manager, ui, ui::UiContainer, ui::UiContent};

// # Containers
// In this example, we learn about the 4 main types of containers provided with mooeye and use them to create a UI containing multiple elements.

/// A very basic scene struct, once again only holding the root element of our GUI.
pub struct DScene {
    /// The root element of DScene's GUI.
    gui: ui::UiElement<()>,
}

impl DScene {
    /// Creates a new DScene.
    /// As you can see, for scenes that do not change based on parameters, we would like to use something like ```DScene::default()``` to
    /// communicate that this is the standard way for a ```DScene``` to come into existence.
    /// However, we cannot derive Default as the passing of a parameter ```ctx: &Context``` is almost always neccessary,
    /// so we have to use ```new(ctx: &Context)``` instead.
    pub fn new(ctx: &mut Context, gfx_ctx: &mut GraphicsContext) -> Result<Self, GameError> {
        // Predefine some visuals so we don't have to do it for every element.

        let vis = ui::Visuals::new(
            Color::from_rgb(180, 120, 60),
            Color::from_rgb(18, 12, 6),
            1.,
            0.,
        );

        // You can also create 'custom' visuals that allow you to set the thickness of each border & radius of each corner separately.

        let vis2 = ui::Visuals::new_custom(
            Color::from_rgb(180, 120, 60),
            Color::from_rgb(18, 12, 6),
            [4., 1., 4., 1.],
            [2., 2., 2., 2.],
        );

        let hover_vis = ui::Visuals::new(
            Color::from_rgb(160, 100, 40),
            Color::from_rgb(18, 12, 6),
            3.,
            0.,
        );

        let cont_vis = ui::Visuals::new_custom(
            Color::from_rgb(60, 120, 180),
            Color::from_rgb(180, 180, 190),
            [16., 8., 8., 8.],
            [12., 2., 2., 12.],
        );

        // Note that the constructor now returns a Result.
        // This is neccessary as the 'add' function used to add UI elements to grid containers can fail, thus failing the constructor.

        // The first container we use is a vertical box simply laying out elements from top to bottom.
        let mut ver_box = ui::containers::VerticalBox::new();
        // We can manually change the spacing between elements in the box
        ver_box.spacing = 10.;
        // first, we need to add all the children to this vertical box.
        // We'll just use TextElement for now, but these can also be images, sprites, placeholders or more containers.
        for i in 0..8 {
            // Create an element.
            let element = graphics::Text::new(format!("{}", i))
                //.set_font("Bahnschrift", 28.)
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(vis2)
                .build();
            // Add the element to the box. This cannot fail for non-grid containers, if ver_box were not an actual container
            // or a container that requires a special method for adding, like e.g. GridBox, it would simply consume the element and do nothing.
            ver_box.add(element);
        }
        // After adding all children, we can convert to a UiElement and style the box like we would style an other element. The usual pattern here is to shadow the variable to avoid use-after-move.
        let ver_box = ver_box
            .to_element_builder(0, ctx)
            .with_visuals(cont_vis)
            // Using larger padding to accomodate our thick borders.
            .with_padding((24., 16., 16., 16.))
            .build();

        // Another container we can use is GridBox. A GridBox needs to be initialized with a set height and width and cannot be extended.
        let mut grid = ui::containers::GridBox::new(4, 4);
        // The contents of a grid box are initialized as empty elements.  We'll add buttons to the diagonal of the grid.
        for i in 0..4 {
            // Create an element.
            let element = graphics::Text::new(format!("{}", i))
                //.set_font("Bahnschrift", 28.)
                .to_owned()
                .to_element_builder(0, ctx)
                .with_visuals(vis)
                // Elements can be given alignment and will align within their respecitive cell in the grid.
                .with_alignment(ui::Alignment::Max, None)
                .build();
            // Add the element to the box. This can fail, if ver_box were not an actual container
            // or a container that requires a special method for adding, like e.g. GridBox.
            grid.add(element, i, i)?;
        }

        // We'll also create our usual back button and put it into the top right of the grid.

        let back = graphics::Text::new("Back!")
            //.set_font("Bahnschrift", 28.)
            .to_owned()
            .to_element_builder(1, ctx)
            .with_visuals(vis)
            .with_hover_visuals(hover_vis)
            .build();

        // This time, we'll enhance our back button a bit by using an icon that is displayed over the top left corner.
        // To achieve this, we'll use a StackBox.
        let mut stack = ui::containers::StackBox::new();
        stack.add(back);
        // The add_top function adds something to the top of a stack box. Creating and adding an element can be done inline.
        if let Ok(image) = graphics::Image::new(ctx, gfx_ctx, "./moo.png") {
            stack.add_top(
                image
                    .to_element_builder(0, ctx)
                    // We'll align the icon to the top right
                    .with_alignment(ui::Alignment::Min, ui::Alignment::Min)
                    // and offset it slightly
                    .with_offset(-10., -10.)
                    .build(),
            )?;
        }

        // to_element is a shorthand for to_element_builder().build() if we want to simply take the default builder and not change anything.
        let stack = stack.to_element(0, ctx);

        // Now, we add the stack to the grid.
        grid.add(stack, 3, 0)?;

        // And finally build the grid.
        let grid = grid
            .to_element_builder(0, ctx)
            .with_visuals(cont_vis)
            .with_padding((24., 16., 16., 16.))
            .build();

        // The horizontal box is exactly the same as the vertical box except for orientation.
        // We will use a horizontal box to contain the boxes created so far.
        // if you don't want to create multiple variables, adding of children can be done inline for non-grid
        // containers by using .with_child.
        Ok(Self {
            gui: ui::containers::HorizontalBox::new()
                .to_element_builder(0, ctx)
                .with_child(ver_box)
                .with_child(grid)
                .build(),
        })
    }
}

impl scene_manager::Scene for DScene {
    fn update(
        &mut self,
        ctx: &mut Context,
        _gfx_ctx: &mut GraphicsContext,
    ) -> Result<scene_manager::SceneSwitch, GameError> {
        // Nothing much to do here, except implement the back button functionality.

        let messages = self.gui.manage_messages(ctx, None);

        if messages.contains(&ui::UiMessage::Triggered(1)) {
            // If it is, we end the current scene (and return to the previous one) by popping it off the stack.
            return Ok(scene_manager::SceneSwitch::pop(1));
        }

        Ok(scene_manager::SceneSwitch::None)
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        gfx_ctx: &mut GraphicsContext,
        mouse_listen: bool,
    ) -> Result<(), GameError> {
        self.gui.draw_to_screen(ctx, gfx_ctx, mouse_listen);

        Ok(())
    }
}
