# MooEye - A ggez-based UI library

This is a very simple UI library to be used with the [ggez game library](https://github.com/ggez/ggez). I wrote this for my own projects and as a challenge. The whole library is very much work-in-progress.

## Features

 * Static UI that communicates with each other via a message-based system.
 * Can use ggez image and text objects as UI elements.
 * Can order elements in multiple types of boxes, alignments, and dynamic sizing.
 * Can add tooltip to elements.
 * Caching of element positions means a recalculation of dynamic element positions is only neccessary when the window size changes or the UI elements themselves change.
 * Transitions system to change layout, look and content of the UI while running. For larger changes of the structure, I suggest a complete re-build of the UI (similar to how an immediate UI rebuilds every frame). Not for performance, but code-readability.
 * Message based communication featuring both internal (clicks) and external (what ever your gamestats wants to tell the UI) messages that can initiate transitions using a customizable message handler.
 * Stack-based scene Manager to make working with multiple scenes in one game significantly easier.
 

 ## TODOs

 * More types of containers.
    * Container that helps with drag & drop
 * Examples & usage guides.
 * More extensive documentation.
 * Possibly docs.rs and crates.io releaes.

 ## Maintenance

 I am maintaining this project mostly for my own purposes. You may not actually want to use this, but hey, it's public. Bug fixes are not guaranteed and features may be lacking.

 ## How to use

 ### Dynamic sizing rules

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