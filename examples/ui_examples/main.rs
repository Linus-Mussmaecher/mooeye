#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

//! # Mooeye - Tests / Examples
//! This is the examples folder. The examples are supposed to be read in order a-g.
//! The examples detail how to set up multiple connected scenes using a scene manager.
//! The different scenes themselves detail different aspects of the mooeye library.
//!
//! You can run the example code (which is also the test code) using ```cargo  test```.

use ggez::*;

/// ## A - Setup
/// This example contains the setup of any mooeye application using the provided SceneManager.
///
/// Except for names, loaded fonts and the specific initial scene, you can copy & paste this into the main method of your own projects.
mod a_setup;

/// ## B: Scene
///
/// This example sets up a very basic scene that pops itself off the stack after two seconds.
///
/// Teaches how to use the ```update``` and ```draw``` functions of ```Scene``` and how to facilitate ```SceneSwitch```es.
mod b_scene;

/// ## C: UI Element
///
/// In this example, we implement our first UI consisting of a single button that returns to the main scene.
///
/// Teaches about ```UiElement```, ```UiElementBuilder```, basic messaging and how to use a ```Scene``` in tandem with a mooeye UI.
mod c_uielement;

/// ## D: Containers
///
/// In this example, we learn about the 4 main types of containers provided with mooeye and use them to create a UI containing multiple elements.
mod d_containers;

/// ## E: Messages & Transitions
///
/// This example introduces messages that can be used for communication between different UI elements (and the game state) as well as transitions that allow you to change the layout, look and content of your UI based on received messages.
/// It creates a UI that can be moved around the screen with its buttons and informs the user of pressed buttons via text.
mod e_messages;
/// ## F: Sprites
///
/// This example explains how to create & draw sprites both in the UI as well as in-game.
mod f_sprites;

/// ## G: Selection Screen
///
/// This example is not really an example, but a selection screen that contains buttons to start any of the previous scenes. Running ```cargo test``` will drop you here, and any of the other examples can be started and tested with their respective buttons.
mod g_selector_scene;

/// Runs the main gui that grants access to the different example uis.
fn main() -> GameResult {
    a_setup::setup_and_run()
}
