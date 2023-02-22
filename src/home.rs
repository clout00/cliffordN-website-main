use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
  html! {
    <main class="container">

      <nav class="navbar">
        <ul class="list">
          <li><p id="logoimg">{"Propulsion Path"}</p></li>
          <li><a href="#">{"About"}</a></li>
          <li><a href="#">{"Products"}</a></li>
          <li><a href="#">{"Contact"}</a></li>
        </ul>
      </nav>

      <section class="main-content">
        <div></div>
      </section>

      <footer class="footer">
        <ul class="footer-info">
        <li><a href="#">{"Contact"}</a></li>
        </ul>
      </footer>

    </main>
  }
}