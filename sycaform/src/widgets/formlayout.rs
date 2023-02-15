use sycamore::prelude::*;

use crate::FormState;

use super::inputform::InputForm;

#[component]
pub fn FormLayout<G: Html>(cx: Scope, formsig: FormState) -> View<G> {
    let fg = formsig.formstate.get().as_ref().clone();

    let form_vec = Vec::from_iter(fg.properties.into_iter());
    let form_signal = create_signal(cx, form_vec);

    view! { cx,
            div(class="col-md-5")
            { div (class="card border-info") {
                div (class="card-header") { h2{"Form from Json Data"}}
                div (class="card-body") {
                    h5 (class="card-title") { (fg.description)}
                     fieldset  {legend  {(fg.title)}

                                     div {
                                          Keyed(
                                            iterable=form_signal,
                                            view=move |cx, x| {

                                                view! { cx,
                                                    InputForm(x)
                                                }
                                            },
                                            key=|x| x.0.clone(),
                                        )
                               }
                    }
                }
                }
            }

    }
}
