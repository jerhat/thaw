mod auto_complete_option;
mod types;

pub use auto_complete_option::AutoCompleteOption;
pub use types::*;

use crate::{
    combobox::listbox::{listbox_keyboard_event, Listbox},
    ComponentRef, Input, InputPrefix, InputRef, InputSuffix,
    _aria::use_active_descendant,
};
use leptos::{context::Provider, either::Either, html, prelude::*};
use std::collections::HashMap;
use thaw_components::{Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style, ArcOneCallback, BoxOneCallback, Model};

#[component]
pub fn AutoComplete(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Input of autocomplete.
    #[prop(optional, into)]
    value: Model<String>,
    /// Autocomplete's placeholder.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    // Whether to clear after selection.
    #[prop(optional, into)] clear_after_select: Signal<bool>,
    /// Whether to blur after selection.
    #[prop(optional, into)]
    blur_after_select: Signal<bool>,
    // On select callback function.
    #[prop(optional, into)] on_select: Option<BoxOneCallback<String>>,
    /// Whether the input is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Size of the input.
    #[prop(optional, into)]
    size: Signal<AutoCompleteSize>,
    #[prop(optional)] auto_complete_prefix: Option<AutoCompletePrefix>,
    #[prop(optional)] auto_complete_suffix: Option<AutoCompleteSuffix>,
    #[prop(optional)] comp_ref: ComponentRef<AutoCompleteRef>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("auto-complete", include_str!("./auto-complete.css"));
    let input_ref = ComponentRef::<InputRef>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let open_listbox = RwSignal::new(false);
    let options = StoredValue::new(HashMap::<String, String>::new());

    let allow_value = move |_| {
        if !open_listbox.get_untracked() {
            open_listbox.set(true);
        }
        true
    };

    let select_option = ArcOneCallback::new(move |option_value: String| {
        if clear_after_select.get_untracked() {
            value.set(String::new());
        } else {
            value.set(option_value.clone());
        }
        if let Some(on_select) = on_select.as_ref() {
            on_select(option_value);
        }

        open_listbox.set(false);
        if blur_after_select.get_untracked() {
            if let Some(input_ref) = input_ref.get_untracked() {
                input_ref.blur();
            }
        }
    });

    let (set_listbox, active_descendant_controller) =
        use_active_descendant(move |el| el.class_list().contains("thaw-auto-complete-option"));
    let on_blur = {
        let active_descendant_controller = active_descendant_controller.clone();
        move |_| {
            active_descendant_controller.blur();
            open_listbox.set(false);
        }
    };
    let on_keydown = {
        let select_option = select_option.clone();
        move |e| {
            let select_option = select_option.clone();
            listbox_keyboard_event(
                e,
                open_listbox,
                false,
                &active_descendant_controller,
                move |option| {
                    options.with_value(|options| {
                        if let Some(value) = options.get(&option.id()) {
                            select_option(value.clone());
                        }
                    });
                },
            );
        }
    };

    comp_ref.load(AutoCompleteRef { input_ref });

    view! {
        <crate::_binder::Binder>
            <div class=class_list!["thaw-auto-complete", class] on:keydown=on_keydown>
                <Input
                    value
                    placeholder
                    disabled
                    on_focus=move |_| open_listbox.set(true)
                    on_blur=on_blur
                    allow_value
                    size=Signal::derive(move || size.get().into())
                    comp_ref=input_ref
                >
                    <InputPrefix if_=auto_complete_prefix.is_some() slot>

                        {if let Some(auto_complete_prefix) = auto_complete_prefix {
                            Some((auto_complete_prefix.children)())
                        } else {
                            None
                        }}

                    </InputPrefix>
                    <InputSuffix if_=auto_complete_suffix.is_some() slot>

                        {if let Some(auto_complete_suffix) = auto_complete_suffix {
                            Some((auto_complete_suffix.children)())
                        } else {
                            None
                        }}

                    </InputSuffix>
                </Input>
            </div>
            <Follower
                slot
                show=open_listbox
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
                auto_height=true
            >
                <Provider value=AutoCompleteInjection {
                    value,
                    select_option,
                    options,
                }>
                    <Listbox set_listbox listbox_ref class="thaw-auto-complete__listbox">
                        {if let Some(children) = children {
                            Either::Left(children())
                        } else {
                            Either::Right(())
                        }}
                    </Listbox>
                </Provider>
            </Follower>
        </crate::_binder::Binder>
    }
}
