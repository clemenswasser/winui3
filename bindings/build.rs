fn main() {
    windows::build!(
        Windows::ApplicationModel::Activation::*,
        Microsoft::UI::Xaml::*,
        Microsoft::UI::Xaml::Controls::TextBox,
    );
}
