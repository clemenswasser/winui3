#![windows_subsystem = "windows"]

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, Controls::TextBox,
        LaunchActivatedEventArgs, Window,
    },
};
use windows::{implement, initialize_sta};

#[implement(
    extend Microsoft::UI::Xaml::Application,
    override OnLaunched
)]
struct App();

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::Current()?;
        window.SetContent(TextBox::new()?)?;
        window.Activate()
    }
}

fn main() -> windows::Result<()> {
    initialize_sta()?;
    
    Application::Start(ApplicationInitializationCallback::new(|_| {
        App().new()?;
        Ok(())
    }))
}
