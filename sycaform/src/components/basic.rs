use sycamore::prelude::*;

#[component]
pub fn BasicForm<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
                        section (class="py-5") {div (class="container px-4 px-lg-5 my-5") {
                    h1 (class="display-3 text-center py-5") {"Basic Form"}

                    div (class="row gx-2 gx-lg-2 align-items-center") {
                        div (class="col-md-9") {
                            div (class="card-group") {
                                div (class="card") {
                                    div (class="card-body") {
                                        fieldset  {legend  {"Fill the Form"}

                                            div (class="mb-3") {
                                                label (class="form-label", for="fname") {"First Name"}

                                                input (class="form-control", id="fname", type="text", aria-describedby="emailHelp") {}
    div (class="form-text", id="emailHelp") {"We'll never share your email with anyone else."}

                                            }

                                            div (class="mb-3") {
                                                label (class="form-label", for="lname") {"Last Name"}

                                                input (class="form-control", id="lname", type="text") {}
    }

                                            div (class="mb-3") {
                                                label (class="form-label", for="tel") {"Telephone"}

                                                input (class="form-control", id="tel", type="text") {}
    }

                                            div (class="mb-3") {
                                                label (class="form-label", for="tel") {"Email"}

                                                input (class="form-control", id="tel", type="email") {}
    }

                                            fieldset  {legend  {"Password"}

                                                div (class="mb-3") {input (class="form-control", type="password", placeholder="Password") {}
    }

                                                div (class="mb-3") {input (class="form-control", type="password", placeholder="Repeat password") {}
    }

                                                div (class="mb-3 form-check form-check-inline") {
                                                    input (class="form-check-input", id="inlineCheckbox1", type="checkbox", value="readonly") {}
    label (class="form-check-label", for="inlineCheckbox1") {"Read Only"}

                                                }

                                                div (class="mb-3 form-check form-check-inline") {
                                                    input (class="form-check-input", id="inlineCheckbox2", type="checkbox", value="disable") {}
    label (class="form-check-label", for="inlineCheckbox2") {"Disable"}

                                                }

                                            }
    }
    button (class="btn btn-primary", type="submit") {"Submit"}

                                    }

                                }

                                div (class="card border-light") {
                                    div (class="card-header text-center") {"Options"}

                                    div (class="card-body px-5") {
                                        div (class="form-check") {
                                            input (class="form-check-input", id="gridRadios1", type="radio", name="gridRadios", value="sanitize", checked=false) {}
    label (class="form-check-label", for="gridRadios1") {"Sanitize String"}

                                        }

                                        div (class="form-check") {
                                            input (class="form-check-input", id="gridRadios2", type="radio", name="gridRadios", value="alpha_only") {}
    label (class="form-check-label", for="gridRadios2") {"Alphabets Only"}

                                        }

                                        div (class="form-check") {
                                            input (class="form-check-input", id="gridRadios2", type="radio", name="gridRadios", value="maximum_string") {}
    label (class="form-check-label", for="gridRadios2") {"Maximum Number of Strings"}

                                        }

                                        div (class="form-check form-switch") {
                                            input (class="form-check-input", id="flexSwitchCheckDefault", type="checkbox", role="switch") {}
    label (class="form-check-label", for="flexSwitchCheckDefault") {"Disable Form"}

                                        }

                                    }

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
