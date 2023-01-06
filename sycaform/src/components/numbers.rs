use sycamore::prelude::*;

#[component]
fn NumberForm<G: Html>(cx: Scope) -> View<G> {
    view! { cx,

                        section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
                    h1 (class="display-3 text-center py-5") {"Numbers Form"}

                    div (class="row gx-2 gx-lg-2 align-items-center") {
                        div (class="col-md-9") {
                            div (class="card-group") {
                                div (class="card") {
                                    div (class="card-body") {
                                        fieldset  {legend  {"Numbers"}

                                            div (class="mb-3") {
                                                label (class="form-label", for="fname") {"Integer"}

                                                input (class="form-control", id="fname", type="number", aria-describedby="emailHelp") {}
    }

                                            div (class="mb-3") {
                                                label (class="form-label", for="fname") {"Float"}

                                                input (class="form-control", id="fname", type="number", step="0.01", aria-describedby="emailHelp") {}
    }

                                            div (class="mb-3") {
                                                label (class="form-label", for="customRange1") {"Number range"}

                                                input (class="form-range", id="customRange1", type="range") {}
    }

                                            div (class="mb-3") {
                                                label (class="form-label", for="customRange3") {"Number range with steps"}

                                                input (class="form-range", id="customRange3", type="range", min="0", max="10", step="2") {}
    }

                                            div (class="mb-3") {
                                                label (class="form-label", for="lname") {"List Group"}

                                                input (class="form-control", id="lname", type="text") {}
    }

                                        }
    }

                                    button (class="btn btn-primary", type="submit") {"Submit"}

                                }

                            }

                        }

                        div (class="col-md-3") {
                            div (class="card border-info") {
                                div (class="card-header") {"Output"}

                                div (class="card-body") {
                                    h5 (class="card-title") {"Info card title"}

                                    p (class="card-text") {"Some quick example text to build on the card title and make up the bulk of the card's content."}

                                }

                            }

                        }

                    }

                }

            }
        }
}
