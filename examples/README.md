# Examples

This is the examples folder. The examples are supposed to be read in order a-g.

You can run the example code (which is also the test code) using ```cargo  test```.

## A: Setup

This example contains the setup of any mooeye application using the provided SceneManager.

Except for names, loaded fonts and the specific initial scene, you can copy & paste this into the main method of your own projects.

## B: Scene

This example sets up a very basic scene that pops itself off the stack after two seconds.

Teaches how to use the ```update``` and ```draw``` functions of ```Scene``` and how to facilitate ```SceneSwitch```es.

## C: UI Element

In this example, we implement our first UI consisting of a single button that returns to the main scene.

Teaches about ```UiElement```, ```UiElementBuilder```, basic messaging and how to use a ```Scene``` in tandem with a mooeye UI.

## D: Containers

Not yet implemented.

## E: Basic Elements

Not yet implemented.

## F: Messages

Not yet implemented.

## G: Transitions

This example introduces transitions that allow you to change the layout, look and content of your UI on the fly. It creates a UI that can be moved around the screen with its buttons and informs the user of pressed buttons via text.

## I: Selection Screen

This example is not really an example, but a selection screen that contains buttons to start any of the previous scenes. Running ```cargo test``` will drop you here, and any of the other examples can be started and tested with their respective buttons.