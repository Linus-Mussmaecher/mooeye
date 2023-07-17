/// There is nothing actually here, because the basic elements Text, Image and Empty are created by simply implementing UiContent on ggez's Text and Image as well as the basic ().
mod empty_element;
/// Contains the implementation of the [crate::UiElement] trait for [ggez::graphics::Image].
mod image_element;
/// Contains the implementation of the [crate::UiElement] trait for [ggez::graphics::Text].
mod text_element;
