use sycamore::prelude::*;

#[component]
fn ArrayForm<G: Html>(cx: Scope) -> View<G> {
    let counter = use_context::<RcSignal<i32>>(cx);

    view! { cx,

            section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
                    h1 (class="display-3 text-center py-5") {"Array Form"}

                    div (class="row gx-2 gx-lg-2 align-items-center") {
                        div (class="col-md-9") {
                            div (class="card-group") {
                                div (class="card") {
                                    div (class="card-body") {
                                        fieldset  {legend  {"Array"}

                                            div (class="mb-3") {
                                                label (class="form-label", for="fname") {"First Name"}

                                                input (class="form-control", id="fname", type="text", aria-describedby="emailHelp") {}
    div (class="form-text", id="emailHelp") {"We'll never share your email with anyone else."}

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
