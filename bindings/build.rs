fn main() {
    windows::build!(
        Windows::ApplicationModel::Activation::*,
        Microsoft::UI::Xaml::{
            Application, ApplicationInitializationCallback, Controls::TextBox,
            LaunchActivatedEventArgs, Window,
        },
    );
}
