use std::{borrow::Cow, ops::Deref};

use leptos::{html::ElementDescriptor, *};
use web_sys::HtmlInputElement;

use crate::{
    prelude::{Callable, Callback},
    Margin, OptionalMaybeSignal,
};

// TODO: Consider merging this component with DateTimeInput if that does not impose a performance regression when using standard text inputs.
pub enum InputType {
    Text,
    Password,
    Number,
}

fn autofocus<T: ElementDescriptor + Clone + Deref<Target = web_sys::HtmlInputElement> + 'static>(
    cx: Scope,
    node_ref: NodeRef<T>,
) {
    node_ref.on_load(cx, move |elem| {
        let outcome = elem.focus();
        if let Err(err) = outcome {
            tracing::error!(?err, "Could not update autofocus.");
        }
    });
}

fn use_focus<T: ElementDescriptor + Clone + Deref<Target = web_sys::HtmlInputElement> + 'static>(
    cx: Scope,
    focus: Signal<bool>,
    node_ref: NodeRef<T>,
) {
    create_effect(cx, move |_prev| {
        let focus = focus.get(); // tracks
        let elem = node_ref.get(); // tracks
        if let Some(elem) = elem {
            let outcome = match focus {
                true => elem.focus(),
                false => elem.blur(),
            };
            if let Err(err) = outcome {
                tracing::error!(?err, "Could not update focus to {}.", focus);
            }
        }
    });
}

#[component]
pub fn Input<S>(
    cx: Scope,
    #[prop(optional, default=InputType::Text)] ty: InputType,
    #[prop(optional, into)] label: OptionalMaybeSignal<String>,
    #[prop(into)] get: MaybeSignal<String>,
    set: S,
    #[prop(optional, into)] prepend: OptionalMaybeSignal<View>,
    #[prop(into, optional)] id: Option<AttributeValue>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    #[prop(into, optional)] should_be_focused: Option<Signal<bool>>,
    #[prop(into, optional)] on_focus_change: Option<Callback<bool>>,
    #[prop(optional)] margin: Option<Margin>,
) -> impl IntoView
where
    S: Fn(String) + Clone + 'static,
{
    let class = class
        .map(|it| Cow::Owned(format!("leptonic-input {it}")))
        .unwrap_or(Cow::Borrowed("leptonic-input"));
    let style = margin.map(|it| format!("--margin: {it}"));

    let ty_str = match ty {
        InputType::Text => "text",
        InputType::Password => "password",
        InputType::Number => "number",
    };

    let node_ref: NodeRef<leptos::html::Input> = create_node_ref(cx);

    if let Some(focus) = should_be_focused {
        use_focus(cx, focus, node_ref);
    }

    let set_clone = set.clone();

    view! { cx,
        <leptonic-input-field style=style>
            <input
                node_ref=node_ref
                id=id
                class=class
                placeholder=move || match &label.0 {
                    Some(label) => std::borrow::Cow::Owned(label.get()),
                    None => std::borrow::Cow::Borrowed(""),
                }
                type=ty_str
                prop:disabled=move || disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
                prop:value=move || get.get()
                on:change=move |e| set(event_target::<HtmlInputElement>(&e).value())
                on:keyup=move |e| set_clone(event_target::<HtmlInputElement>(&e).value())
                on:blur=move |_e| { on_focus_change.map(|cb| cb.call(false)); }
                on:focus=move |_e| { on_focus_change.map(|cb| cb.call(true)); }
            />

            {match prepend.0 {
                Some(view) => view! { cx,
                    <div>
                        { view.get() }
                    </div>
                }.into_view(cx),
                None => ().into_view(cx),
            }}

            //<LeptosIcon icon=icon />
        </leptonic-input-field>
    }
}
