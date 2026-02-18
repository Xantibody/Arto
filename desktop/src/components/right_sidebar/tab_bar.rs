use dioxus::prelude::*;

use crate::components::icon::{Icon, IconName};

use super::RightSidebarTab;

#[component]
pub fn TabBar(
    active_tab: RightSidebarTab,
    on_change: EventHandler<RightSidebarTab>,
    on_close: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "right-sidebar-tabs",

            // Contents tab
            button {
                class: if active_tab == RightSidebarTab::Contents { "right-sidebar-tab active" } else { "right-sidebar-tab" },
                onclick: move |_| on_change.call(RightSidebarTab::Contents),
                span { "Contents" }
            }

            // Search tab
            button {
                class: if active_tab == RightSidebarTab::Search { "right-sidebar-tab active" } else { "right-sidebar-tab" },
                onclick: move |_| on_change.call(RightSidebarTab::Search),
                span { "Search" }
            }

            // Close button (right-aligned via margin-left: auto)
            button {
                class: "sidebar-panel-close-button",
                title: "Close Right Sidebar",
                onclick: move |_| on_close.call(()),
                Icon {
                    name: IconName::SidebarRightCollapse,
                    size: 18,
                }
            }
        }
    }
}
