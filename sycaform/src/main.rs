use components::basic::BasicForm;
use sycamore::prelude::*;
pub mod components;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    // let state = create_signal(cx, 0i32);
    // let increment = |_| state.set(*state.get() + 1);
    // let decrement = |_| state.set(*state.get() - 1);
    // let reset = |_| state.set(0);
    view! { cx,
            // Body Start
            nav (class="navbar navbar-expand-lg navbar-light bg-light") {div (class="container px-4 px-lg-5") {
                    a (class="navbar-brand", href="#!") {"Sycamore Data Form"}

                    button (class="navbar-toggler", type="button", data-bs-toggle="collapse", data-bs-target="#navbarSupportedContent", aria-controls="navbarSupportedContent", aria-expanded="false", aria-label="Toggle navigation") {span (class="navbar-toggler-icon") {}
    }

                    div (class="collapse navbar-collapse", id="navbarSupportedContent") {
                        ul (class="navbar-nav me-auto mb-2 mb-lg-0 ms-lg-4") {li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Home"} }

                            li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Nested"} }

                            li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Arrays"} }


                            li (class="nav-item") {
                                a (class="nav-link active", aria-current="page", href="#!") {"File"}
                            }

                        }
    }

                }

            }
           BasicForm()
                // Body End
        }
}

fn main() {
    sycamore::render(App);
}
