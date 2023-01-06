use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
             nav (class="navbar navbar-expand-lg navbar-light bg-light") {div (class="container px-4 px-lg-5") {
                    a (class="navbar-brand", href="#!") {Start Bootstrap}

                    button (class="navbar-toggler", type="button", data-bs-toggle="collapse", data-bs-target="#navbarSupportedContent", aria-controls="navbarSupportedContent", aria-expanded="false", aria-label="Toggle navigation") {span (class="navbar-toggler-icon") {}
    }

                    div (class="collapse navbar-collapse", id="navbarSupportedContent") {
                        ul (class="navbar-nav me-auto mb-2 mb-lg-0 ms-lg-4") {li (class="nav-item") {a (class="nav-link active", aria-current="page", href="#!") {"Home"}
    }

                        }
    }

                }

            }
    section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
                    div (class="row gx-4 gx-lg-5 align-items-center") {
                        div (class="col-md-6") {
                            ul (class="list-group w-50") {li (class="list-group-item border-0") {
                                    i (class="bi bi-plus-square me-1") {}

                                   " Tree Group 1"
                                }

                                li (class="list-group-item border-0") {
                                    i (class="fas fa-bomb fa-fw me-4") {}

                                   " Leaf Item"
                                }

                            }
    ul (class="list-group border-0") {li (class="list-group-item ba bi-plus-square border-0") {
                                    i (class="fas fa-code fa-fw me-4") {}

                                    "Nested Tree One"
                                }

                                ul (class="list-group") {li (class="list-group-item border-0") {
                                        i (class="far fa-gem fa-fw me-4") {}

                                        "Leaf One"
                                    }

                                    li (class="list-group-item border-0") {
                                        i (class="fas fa-cogs fa-fw me-4") {}

                                        "Leaf two"
                                    }

                                    ul (class="list-group border-0") {li (class="list-group-item ba bi-plus-square border-0") {
                                            i (class="fas fa-code fa-fw me-4") {}

                                           " Nested Tree Two"
                                        }

                                        ul (class="list-group") {li (class="list-group-item border-0") {
                                                i (class="far fa-gem fa-fw me-4") {}

                                                "Leaf One"
                                            }

                                            li (class="list-group-item border-0") {
                                                i (class="fas fa-cogs fa-fw me-4") {}

                                                "Leaf two"
                                            }

                                        }
    }
    }
    }
    }

                        div (class="col-md-6") {
                            h1 (class="display-5 fw-bolder") {"Sycamore Tree View"}

                            p (class="lead") {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Praesentium at dolorem quidem modi. Nam sequi consequatur obcaecati excepturi alias magni, accusamus eius blanditiis delectus ipsam minima ea iste laborum vero?"}

                        }

                    }

                }

            }
    footer (class="py-5 bg-dark") {div (class="container") {p (class="m-0 text-center text-white") {"Copyright © Your Website 2022"}
    }

            }


        }
}

fn main() {
    sycamore::render(App);
}
