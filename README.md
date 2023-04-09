# MooEye - A ggez-based UI library

[![Docs Status](https://docs.rs/mooeye/badge.svg)](https://docs.rs/mooeye)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Linus-Mussmaecher/mooeye/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/mooeye.svg)](https://crates.io/crates/mooeye)
[![Crates.io](https://img.shields.io/crates/d/mooeye.svg)](https://crates.io/crates/mooeye)

This is a very simple UI library building upon [ggez game library](https://github.com/ggez/ggez). It was originally written for personal use with my own Rust gamedev projets. The whole library is very much work-in-progress.

## UI Features

 * Static UI that communicates with itself via a message-based system.
 * Can use ggez image and text objects as UI elements.
 * Can order elements in multiple types of boxes, alignments, and dynamic sizing.
 * Can add tooltip to elements.
 * Caching of element positions means a recalculation of dynamic element positions is only neccessary when the window size changes or the UI elements themselves change.
 * Transitions system to change layout, look and content of the UI while running. For larger changes of the structure, a complete re-build of the UI (similar to how an immediate UI rebuilds every frame) is suggested.
 * Message based communication with both internal (a user clicks a button) and external (a resource amount in your game changes) sources. Messages are handled internally with a customizable message handler, allowing you to change the look and content of your UI to react to user interaction or game state changes. All internal messages are also returned from the message handling function of your top UI element, allowing your game state to react to user inputs.

## Additional Features

 * Stack-based scene manager eases the work with multiple scenes in a single game significantly by handling changes of main game state and appropriate drawing of (stacked) scenes.
 * Sprite struct automates drawing an animated sprite with multiple variants (e.g. an attacking, walking, ... character) and can be loaded from a single spritesheet file.

## Maintenance

I am maintaining this project mostly for my own purposes. You may not actually want to use this, but hey, it's public. Bug fixes are not guaranteed and features ~~may be~~ are lacking, as they are most precisely what I wanted from a UI library, which may not neccessarily be what you want.

## How to use

### UI 

When using MooEye, your game state struct contains an ```gui: UiElement<T>```. Initialize this value with any container and create a tree of [```UIElement```s](https://docs.rs/mooeye/latest/mooeye/ui_element/struct.UiElement.html#) representing the state of your UI. In this step, you also define the Interaction of the UI with both user and game state via [message handlers](https://docs.rs/mooeye/latest/mooeye/ui_element/struct.UiElementBuilder.html#method.with_message_handler) and [transitions](https://docs.rs/mooeye/latest/mooeye/ui_element/struct.Transition.html).

Every frame, you want to call [```gui.draw_to_screen()```](https://docs.rs/mooeye/latest/mooeye/ui_element/struct.UiElement.html#method.draw_to_screen) to draw the Ui within your draw function to draw the UI. ```T``` is the type of your extern messages. Collect every change in your game state that you want to represent in the UI and pass them on to the UI in the update step of your game loop using [```gui.manage_messages()```](https://docs.rs/mooeye/latest/mooeye/ui_element/struct.UiElement.html#method.manage_messages). Your message handlers will then receive these messages and change the UI appropriately (for larger changes, you may also completely rebuild the UI rather than writing enormous handlers). [```gui.manage_messages()```](https://docs.rs/mooeye/latest/mooeye/ui_element/struct.UiElement.html#method.manage_messages) will return a set of internal messages informing you of buttons the user has clicked. This way, your game state can conversely react to interaction with the UI.

For more extensive explanation and examples see the [docs](https://docs.rs/mooeye) or the examples in the [tests folder](/tests/).

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

Creating and using a scene manager is as simple as having your scenes implement [Scene](https://docs.rs/mooeye/latest/mooeye/scene_manager/trait.Scene.html) instead of [Event Handler](https://docs.rs/ggez/latest/ggez/event/trait.EventHandler.html) and starting your game via [SceneManager::new_and_run](https://docs.rs/mooeye/latest/mooeye/scene_manager/struct.SceneManager.html#method.new_and_run) instead of [event::run](https://docs.rs/ggez/latest/ggez/event/fn.run.html).

### Sprites
 
Sprites can be created with a path just like any ggez-Image, but can display animation and multiple states of an object. See the respective documentation in the [sprite documentation](https://docs.rs/mooeye/latest/mooeye/sprite/struct.Sprite.html).

The source image file needs to contain the different frames of each animation cycle aligned horizontally, with the different states forming the rows.

## License

MIT License