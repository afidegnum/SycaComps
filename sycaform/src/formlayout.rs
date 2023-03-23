use serde_json::json;
use serde_json::Value;
use sycamore::prelude::*;

use crate::FormState;

use super::inputform::InputForm;

#[component]
pub fn FormLayout<G: Html>(cx: Scope, formsig: FormState) -> View<G> {
    let fg = formsig.formstate.get().as_ref().clone();
    // let title = fg.schema.get("title").unwrap();
    // let title = fg.schema["title"].clone();
    // let description = fg.schema["description"].clone();

    // // let form_vectors = fg.schema.get("properties");
    // let properties = fg.schema.get("properties").unwrap().as_object().unwrap();
    // let form_vec = Vec::from_iter(properties.into_iter());
    // let form_vec = json!(fg.properties.into_iter());
    let form_vec = Vec::from_iter(fg.merged_schema.into_iter());
    let form_signal = create_signal(cx, form_vec);

    view! { cx,
            div(class="col-md-5")
            { div (class="card border-info") {
                div (class="card-header") { h2{"Form from Json Data"}}
                div (class="card-body") {
                    // h5 (class="card-title") { (fg.schema["description"])}
                     fieldset  {legend  { (fg.schema["title"] ) }

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
