mod home;
use home::App;

fn main() {
    yew::Renderer::<App>::new().render();
}