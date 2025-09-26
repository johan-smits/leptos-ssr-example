use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
struct Site {
    loaded: bool,
    title: String,
    style: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let site: RwSignal<Option<Site>> = RwSignal::new(None);
    provide_context(site);

    let site_load = OnceResource::new_blocking(async move {
        leptos::logging::debug_warn!("Loading the site");
        Site {
            loaded: true,
            title: String::from("Welcome from Leptos site data"),
            style: String::from("example"),
        }
    });
    provide_context(site_load);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/test.css" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute
                        path=StaticSegment("/")
                        view=move || {
                            view! {
                                <Suspense fallback=|| ()>
                                    {move || Suspend::new(async move {
                                        let data = site_load.await;
                                        match data.loaded {
                                            true => {
                                                view! {
                                                    <Title text=data.title.clone() />
                                                    <Outlet />
                                                }
                                                    .into_any()
                                            }
                                            false => panic!("Should reach this point!"),
                                        }
                                    })}
                                </Suspense>
                            }
                        }
                    >
                        <Route path=StaticSegment("") view=Home />
                        <Route path=path!(":page_id") view=Home />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

/// This component does checking on the site style if should apply
#[component]
fn Home() -> impl IntoView {
    let site = expect_context::<OnceResource<Site>>();

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=async move { site.await }
            // the data is bound to whatever variable name you provide
            let:data
        >
            {match data.style.as_str() {
                "example" => view! { <HomePage /> }.into_any(),
                _ => "Invalid style".into_any(),
            }}
        </Await>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
