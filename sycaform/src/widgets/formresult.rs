use sycamore::prelude::*;

use crate::FormData;

#[component]
pub fn FormResult<G: Html>(cx: Scope) -> View<G> {
    let data_context = use_context::<FormData>(cx);

    view! { cx,

         div (class="col-md-3") {
            div (class="card border-info") {
                div (class="card-header") {"Output"}

                div (class="card-body") {
                    h5 (class="card-title") {"Json Form Result"}

                    pre{(format!("{:#?}", data_context.data.get()))}

                }

            }

        }

    }
}
