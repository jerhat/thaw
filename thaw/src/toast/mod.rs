mod toast;
mod toast_body;
mod toast_footer;
mod toast_title;
mod toaster;
mod toaster_provider;

pub use toast::*;
pub use toast_body::*;
pub use toast_footer::*;
pub use toast_title::*;
pub use toaster_provider::*;

use leptos::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender, TryIter};
use tachys::view::any_view::AnyView;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone)]
pub struct ToasterInjection {
    sender: Sender<(AnyView<Dom>, ToastOptions)>,
    trigger: ArcTrigger,
}

impl ToasterInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn channel() -> (Self, ToasterReceiver) {
        let (sender, receiver) = channel::<(AnyView<Dom>, ToastOptions)>();
        let trigger = ArcTrigger::new();

        (
            Self {
                sender,
                trigger: trigger.clone(),
            },
            ToasterReceiver::new(receiver, trigger),
        )
    }

    pub fn dispatch_toast(&self, any_view: AnyView<Dom>, options: ToastOptions) {
        self.sender.send((any_view, options)).unwrap_throw();
        self.trigger.trigger();
    }
}

pub struct ToasterReceiver {
    receiver: Receiver<(AnyView<Dom>, ToastOptions)>,
    trigger: ArcTrigger,
}

impl ToasterReceiver {
    pub fn new(receiver: Receiver<(AnyView<Dom>, ToastOptions)>, trigger: ArcTrigger) -> Self {
        Self { receiver, trigger }
    }

    pub fn try_recv(&self) -> TryIter<'_, (AnyView<Dom>, ToastOptions)> {
        self.trigger.track();
        self.receiver.try_iter()
    }
}
