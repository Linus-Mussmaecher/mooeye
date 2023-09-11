# MooEyeWeb - A GoodWebGame-based UI library

This is the web version of [Mooeye](https://github.com/Linus-Mussmaecher/mooeye), building on [Good Web Game](https://github.com/ggez/good-web-game) instead of [ggez game library](https://github.com/ggez/ggez).

Badges for Mooeye:

[![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/mooeye)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/mooeye/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/mooeye.svg)](https://crates.io/crates/mooeye)
[![Crates.io](https://img.shields.io/crates/d/mooeye.svg)](https://crates.io/crates/mooeye)

Unlike Mooeye, mooeye web is less polished and does not have a cargo released. Some docs may falsely be referring to mooeye or ggez features.
The intent behind mooeye was transporting some of my ggez-based games to the web.

## UI Features

 * Static UI that communicates with itself via a message-based system.
 * Can use ggez image and text objects as UI elements.
 * Can order elements in multiple types of boxes, alignments, and dynamic sizing.
 * Can add tooltip to elements.
 * Caching of element positions means a recalculation of dynamic element positions is only neccessary when the window size changes or the UI elements themselves change.
 * Transitions system to change layout, look and content of the UI while running. For larger changes of the structure, a complete re-build of the UI (similar to how an immediate UI rebuilds every frame) is suggested.
 * Message based communication with both internal (a user clicks a button) and external (a resource amount in your game changes) sources. Messages are handled internally with a customizable message handler, allowing you to change the look and content of your UI to react to user interaction or game state changes. All internal messages are also returned from the message handling function of your top UI element, allowing your game state to react to user inputs.
 * Dynamic chaning of UiGraph by adding and removing elements even after initialisation.

## Additional Features

 * Stack-based scene manager eases the work with multiple scenes in a single game significantly by handling changes of main game state and appropriate drawing of (stacked) scenes.
 * Sprite struct automates drawing an animated sprite with multiple variants (e.g. an attacking, walking, ... character) and can be loaded from a single spritesheet file.
 * Sprite pool struct to make batch-loading and formatting of sprites more ergonomic.

## Maintenance

I am maintaining this project mostly for my own purposes, and by myself. Updates may be far and in between, but Mooeye is in a very usable state right now.

## How to use

### UI 

When using MooEye, your game state struct should contain a ```gui: UiElement<T>```. Initialize this value with any container and create a tree of [```UIElement```s](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElement.html#) representing the state of your UI. In this step, you also define the Interaction of the UI with both user and game state via [message handlers](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElementBuilder.html#method.with_message_handler) and [transitions](https://docs.rs/mooeye/latest/mooeye/ui/struct.Transition.html).

Every frame, you want to call [```gui.draw_to_screen()```](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElement.html#method.draw_to_screen) to draw the Ui within your draw function to draw the UI. ```T``` is the type of your extern messages. Collect every change in your game state that you want to represent in the UI and pass them on to the UI in the update step of your game loop using [```gui.manage_messages()```](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElement.html#method.manage_messages). Your message handlers will then receive these messages and change the UI appropriately (for larger changes, you may also completely rebuild the UI rather than writing enormous handlers). [```gui.manage_messages()```](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElement.html#method.manage_messages) will return a set of internal messages informing you of buttons the user has clicked. This way, your game state can conversely react to interaction with the UI.

Additionally, your update function can interact with your UI by calling [```gui.add_elements()```](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElement.html#method.add_element) and [```gui.remove_elements()```](https://docs.rs/mooeye/latest/mooeye/ui/struct.UiElement.html#method.remove_elements) to add and remove elements based on their id and change your entire UI layout.

For more extensive explanation and examples see the [docs](https://docs.rs/mooeye) or the examples in the [examples folder](/examples/ui_examples).

### Dynamic sizing rules

The following rules guide how an element in MooEye tries to size itself.

#### Outer bounds:

 * Any element will always stay within the bound given by its ``layout::size``.
 * If the rectangle that ``draw_to_rectangle`` is called with is too small, the element will not display (but may still consume space).
 * If the given rectangle is within the bounds, the element will fit to it accoding to the ``layout::size`` modifiers
 * If the given rectangle is too large, the element will not grow above its ``layout::size`` and instead align as according to its ``layout::alignment``.

#### Inner bounds:

 * An additional size requirement may be given not by the ``layout``, but by the ``content`` of the element. Here, the upper bounds will (so far) be ignored, but the element will try to respect the lower bounds when adjusting its size. These content bounds are mostly used to containers respect the space requirements of their children.
 * If bounds are too small for this content limit, the element will still fit itself to the bounds and deal with this size limitation accordingly. E.g., a container might give too-small rectangles to its children, causing some of them to not display as described above.
 * If bounds are too large for this conent limit, the element will still fit itself to the bounds and deal with the unneeded space accordingly. E.g., a container might give too-large rectangles to its children, causing them too act as described above.

#### Preserve ratio:

Elements with the ``preserve_ratio`` flag of their ``layout`` set to true will only display their content in the ratio of the lower limits of their ``layout::size``. Their background will be drawn as normal, and the element will then scale down in the dimension that would have been stretched more in order to fit onto this background.

### Scene Manager

Creating and using a [scene manager](https://docs.rs/mooeye/latest/mooeye/scene_manager/struct.SceneManager.html) is as simple as having your scenes implement [Scene](https://docs.rs/mooeye/latest/mooeye/scene_manager/trait.Scene.html) instead of [Event Handler](https://docs.rs/ggez/latest/ggez/event/trait.EventHandler.html) and starting your game via [SceneManager::new_and_run](https://docs.rs/mooeye/latest/mooeye/scene_manager/struct.SceneManager.html#method.new_and_run) instead of [event::run](https://docs.rs/ggez/latest/ggez/event/fn.run.html).

See also the examples in the [examples folder](/examples/) for usage of the SceneManager.

### Sprites
 
Sprites can be created with a path just like any ggez-Image, but can display animation and multiple states of an object. See the respective documentation in the [sprite documentation](https://docs.rs/mooeye/latest/mooeye/sprite/struct.Sprite.html).

The source image file needs to contain the different frames of each animation cycle aligned horizontally, with the different states forming the rows.

See also the relevant examples in the [examples folder](/examples/) for usage of Sprite.

Entire folders of sprites can be loaded with a [SpritePool](https://docs.rs/mooeye/latest/mooeye/sprite/struct.SpritePool.html), making repeated acceses to the file system unneccessary.

## License

MIT License
