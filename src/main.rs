use yew::prelude::*;
use yew_router::prelude::*;
use yew::prelude::*;

#[function_component(Login)]
fn login() -> Html {
    html! {
        <div class="bg-gray-500 h-screen flex items-center justify-center">
            <div class="bg-white p-10 rounded-lg shadow-md w-96">
                <h2 class="text-2xl font-bold mb-6">{"Login"}</h2>
                <form>
                    <div class="mb-4">
                        <label for="email" class="block text-gray-700 text-sm font-bold mb-2">{"Email Address"}</label>
                        <input type="email" id="email" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" placeholder="Enter Email Address"/>
                    </div>
                    <div class="mb-6">
                        <label for="password" class="block text-gray-700 text-sm font-bold mb-2">{"Password"}</label>
                        <input type="password" id="password" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" placeholder="Enter Password"/>
                    </div>
                    <div class="flex items-center justify-between">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline" type="button">
            {"Sign In"}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}


#[function_component(Sidebar)]
fn sidebar() -> Html {
    html! {
        <div class="bg-gray-800 text-white h-screen w-64 flex flex-col">
            <div class="flex items-center justify-center h-20 bg-gray-900">
                <h1 class="text-2xl font-bold">{"My App"}</h1>
            </div>
            <nav class="flex flex-col mt-8">
                <a href="#" class="px-4 py-2 hover:bg-gray-700">{"Home"}</a>
                <a href="#" class="px-4 py-2 hover:bg-gray-700">{"Profile"}</a>
                <a href="#" class="px-4 py-2 hover:bg-gray-700">{"Settings"}</a>
            </nav>
        </div>
    }
}

struct TeamMember {
    name: &'static str,
    role: &'static str,
    image: &'static str,
}
#[function_component(TeamSection)]
fn team_section() -> Html {
    let team_members = vec![
        TeamMember {
            name: "Alice Johnson",
            role: "CEO",
            image: "alice.jpg",
        },
        TeamMember {
            name: "Bob Smith",
            role: "CTO",
            image: "bob.jpg",
        },
        TeamMember {
            name: "Charlie Brown",
            role: "CFO",
            image: "charlie.jpg",
        },
        TeamMember {
            name: "David Lee",
            role: "CMO",
            image: "david.jpg",
        },
        TeamMember {
            name: "Emily Wilson",
            role: "COO",
            image: "emily.jpg",
        },
    ];

    html! {
        <div class="bg-gray-100 py-16">
            <h2 class="text-3xl font-bold text-center mb-10">{"Our Team"}</h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-8">
                {for team_members.iter().map(|member| {
                    html! {
                        <div class="bg-white rounded-lg shadow-md p-6 text-center">
                            <img src={member.image} alt={member.name} class="w-24 h-24 mx-auto rounded-full mb-4" />
                            <h3 class="text-lg font-bold">{member.name}</h3>
                            <p class="text-gray-500">{member.role}</p>
                        </div>
                    }
                })}
            </div>
        </div>
    }


}

use yew::prelude::*;

#[function_component(ChatApp)]
fn chat_app() -> Html {
    html! {
        <div class="flex h-screen">
            <div class="bg-gray-800 w-64 text-white p-4">
                <h2 class="text-xl font-bold mb-4">{"Rocket Chat"}</h2>
                <nav>
                    <ul>
                        <li><a href="#" class="block py-2 px-4 hover:bg-gray-700">{"Lobby"}</a></li>
                        <li><a href="#" class="block py-2 px-4 hover:bg-gray-700">{"Rocket"}</a></li>
                        <li><a href="#" class="block py-2 px-4 hover:bg-gray-700">{"New Room"}</a></li>
                    </ul>
                </nav>
            </div>
            <div class="flex-1 p-4">
                <div class="bg-gray-900 text-white p-4 rounded-lg mb-4">
                    <h2 class="text-xl font-bold">{"Rocket"}</h2>
                    <div class="h-full overflow-y-auto">
                        <p>{"This is another room. Neat, huh?"}</p>
                    </div>
                </div>
                <div class="flex">
                    <input type="text" class="flex-1 p-2 border border-gray-300 rounded-l-lg" placeholder="Send a message..." />
                    <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-r-lg">{"Send"}</button>
                </div>
            </div>
        </div>
    }
}

#[function_component(Dashboard)]
fn dashboard() -> Html {
    html! {
        <div class="bg-gray-100 h-screen p-8">
            <div class="bg-white p-6 rounded-lg shadow-md mb-8">
                <h2 class="text-2xl font-bold mb-4">{"Overview"}</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <h3 class="text-xl font-bold mb-2">{"Total Sales"}</h3>
                        <p class="text-4xl font-bold">{"$10,000"}</p>
                    </div>
                    <div>
                        <h3 class="text-xl font-bold mb-2">{"Total Users"}</h3>
                        <p class="text-4xl font-bold">{"1000"}</p>
                    </div>
                </div>
                           </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                <div class="bg-white p-6 rounded-lg shadow-md">
                    <h3 class="text-xl font-bold mb-4">{"Recent Activity"}</h3>
                    <ul>
                        <li>{"User A purchased Product X"}</li>
                        <li>{"User B returned Product Y"}</li>
                                            </ul>
                </div>
                <div class="bg-white p-6 rounded-lg shadow-md">
                    <h3 class="text-xl font-bold mb-4">{"User Insights"}</h3>
                    <p>{"Placeholder for a chart or table"}</p>
                   // {/* Replace with actual user insights, e.g., age distribution, geographic location */}
                </div>
            </div>
        </div>
    }
}


// ===================================================================================
// for {username}.github.io/{repo_name}
// replace 'yew-template-for-github.io' to your repo name

#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
    #[at("/miniature-train/")]
    Home,
    #[at("/miniature-train/:s")]
    Route,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/miniature-train/about")]
    About,
    #[not_found]
    #[at("/miniature-train/404")]
    NotFound,
}

fn root_route(routes: &RootRoute) -> Html {
    match routes {
        RootRoute::Home => html! {
            <div class={classes!("bg-indigo-500")}>
            <p class={classes!("text-4xl")}>{ "miniature-train" }</p>
            <Login/>
            <p>{"sidebar"}</p>
            <Sidebar/>
            <p>{"TEAms"}</p>
            <TeamSection/>
            <Dashboard/>
            <ChatApp/>
            </div>
        },
        RootRoute::Route => html! {
            <Switch<Route> render={Switch::render(switch)} />
        },
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::About => html! { <p>{ "About" }</p> },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

// ===================================================================================
// for {username}.github.io

// #[derive(Clone, Routable, PartialEq)]
//  enum RootRoute {
//      #[at("/")]
//      Home,
//      #[at("/about")]
//      About,
//      #[not_found]
//      #[at("/404")]
//      NotFound,
//  }

//  fn root_route(routes: &Route) -> Html {
//      match routes {
//          RootRoute::Home => html! { <p class="text-4xl">{ "Yew Template" }</p> },
//          RootRoute::About => html! { <p>{ "About" }</p> },
//          RootRoute::NotFound => html! { <p>{ "Not Found" }</p> },
//      }
//  }

// ===================================================================================




/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        // ********************************************************
        // **    basename is not supported on yew 0.19.0 yet.    **
        // <BrowserRouter basename="/yew-template-for-github-io/">
        //     <Switch<Route> render={Switch::render(switch)} />
        // </BrowserRouter>
        // ********************************************************
        <BrowserRouter>
            <Switch<RootRoute> render={Switch::render(root_route)} />
            <p>{"Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat."}</p>
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::start_app::<App>();
}
