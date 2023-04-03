# MooEye - A ggez-based UI library

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
 

 ## TODOs

 * Drag & Drop
 * Overhaul the current tooltip implementation using z-levels.
 * Examples & usage guides.
 * More extensive documentation.
 * Possibly docs.rs and crates.io releaes.

 ## Maintenance

 I am maintaining this project mostly for my own purposes. You may not actually want to use this, but hey, it's public. Bug fixes are not guaranteed and features ~~may be~~ are lacking, as they are most precisely what I wanted from a UI library, which may not neccessarily be what you want.

 ## How to use

 TODO

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

 ## License

MIT License