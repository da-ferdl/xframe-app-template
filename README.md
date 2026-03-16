# Xframe example / basic template

Example of the for egui usage on macOS / iOS / Android - with XFrame, a variation of Eframe, from the egui-fork.

ATTENTION: The used egui-fork has `Context` !Send + !Sync, so Rc and RefCell could be used instead of Arc and Mutex. So egui third party crates need to be forked with the egui dependency on the own fork.

Uses various own forks, look at the dependencies!<br>
This is a rough, ideas thrown together project to have basic examples of the various components.

This example contains the `InteractRectHandler` to update / accept input events only when needed (eg. moving over elements that have a sense).<br>
This works good, but is cumbersome because all "interactive" elements need to be registered.<br>
But the idea is interesting for usage eg. for automotive, where when a CarPlay or Camera display is on top of the app, to exclude interaction on that inner area, but allow interaction on the surrounding - egui - elements.

Additionally included:
- Theme / styling example
- Custom virtual keyboard on android - on android the system keyboard does not work
- Extensions on egui `Context`
- Layout Preference, ...
- UiEventProxy
- Router - with examples of different layouts for desktop / mobile
- Custom fonts usage
- XCodegen to create / recreate XCode project based on yaml definitions - see _ios directory
- Android safe area - see _android directory, MainActivity.java
- build rust and target libs from within Android Gradle / XCode build
