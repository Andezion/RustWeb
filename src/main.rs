use leptos::prelude::*;

// use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="bg-gray-100 font-sans antialiased">
            <Navigation />
            <Hero />
            <FeaturedWorkouts />
            <ProgressTracking />
            <Testimonials />
            <CallToAction />
            <Footer />
        </div>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = create_signal(false);

    view! {
        <nav class="bg-white shadow-lg sticky top-0 z-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex items-center">
                        <div class="flex-shrink-0 flex items-center">
                            <svg class="text-red-600 h-8 w-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                            </svg>
                            <span class="ml-2 text-xl font-bold text-slate-800">"IronPulse"</span>
                        </div>
                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                            <a href="#" class="border-red-600 text-red-600 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">"Home"</a>
                            <a href="#" class="border-transparent text-slate-800 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">"Training"</a>
                            <a href="#" class="border-transparent text-slate-800 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">"Nutrition"</a>
                            <a href="#" class="border-transparent text-slate-800 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">"Community"</a>
                        </div>
                    </div>
                    <div class="hidden sm:ml-6 sm:flex sm:items-center">
                        <button class="bg-red-600 px-4 py-2 rounded-md text-white text-sm font-medium hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-600">
                            "Sign In"
                        </button>
                    </div>
                    <div class="-mr-2 flex items-center sm:hidden">
                        <button
                            on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                            type="button"
                            class="inline-flex items-center justify-center p-2 rounded-md text-slate-800 hover:text-red-600 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-red-600"
                        >
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <div class="relative bg-slate-800 overflow-hidden" style="clip-path: polygon(0 0, 100% 0, 100% 85%, 0 100%);">
            <div class="absolute inset-0 bg-gradient-to-br from-slate-900 to-slate-700"></div>
            <div class="relative max-w-7xl mx-auto py-24 px-4 sm:py-32 sm:px-6 lg:px-8">
                <div class="text-center">
                    <h1 class="text-4xl font-extrabold tracking-tight text-white sm:text-5xl lg:text-6xl">
                        "Fuel Your " <span class="text-red-600">"Athletic Beast"</span>
                    </h1>
                    <p class="mt-6 max-w-lg mx-auto text-xl text-gray-300">
                        "Personalized training programs, nutrition plans, and community support to help you crush your fitness goals."
                    </p>
                    <div class="mt-10 flex justify-center space-x-4">
                        <a href="#" class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-600">
                            "Get Started"
                        </a>
                        <a href="#" class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-slate-700 hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-600">
                            <svg class="mr-2 h-5 w-5" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M8 5v14l11-7z" />
                            </svg>
                            " Watch Demo"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn FeaturedWorkouts() -> impl IntoView {
    let workouts = vec![
        Workout {
            title: "Power Builder Program".to_string(),
            description: "Gain strength and muscle with this progressive overload program."
                .to_string(),
            category: "Strength".to_string(),
            duration: "12 weeks".to_string(),
            coach: "Coach Johnson".to_string(),
            rating: 5.0,
            bg_color: "bg-red-100".to_string(),
            text_color: "text-red-800".to_string(),
        },
        Workout {
            title: "Fat Burn Blast".to_string(),
            description: "High intensity intervals to torch calories and boost metabolism."
                .to_string(),
            category: "HIIT".to_string(),
            duration: "8 weeks".to_string(),
            coach: "Coach Maria".to_string(),
            rating: 4.9,
            bg_color: "bg-green-100".to_string(),
            text_color: "text-green-800".to_string(),
        },
        Workout {
            title: "Flexible Warrior".to_string(),
            description: "Improve flexibility and prevent injuries with daily mobility work."
                .to_string(),
            category: "Mobility".to_string(),
            duration: "4 weeks".to_string(),
            coach: "Coach Alex".to_string(),
            rating: 4.8,
            bg_color: "bg-purple-100".to_string(),
            text_color: "text-purple-800".to_string(),
        },
    ];

    view! {
        <div class="py-12 bg-white">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="lg:text-center">
                    <h2 class="text-base text-red-600 font-semibold tracking-wide uppercase">"Workout Programs"</h2>
                    <p class="mt-2 text-3xl leading-8 font-extrabold tracking-tight text-slate-800 sm:text-4xl">
                        "Train Like a Champion"
                    </p>
                </div>

                <div class="mt-10">
                    <div class="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
                        <For
                            each=move || workouts.clone()
                            key=|workout| workout.title.clone()
                            children=move |workout| {
                                view! { <WorkoutCard workout=workout /> }
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug)]
struct Workout {
    title: String,
    description: String,
    category: String,
    duration: String,
    coach: String,
    rating: f32,
    bg_color: String,
    text_color: String,
}

#[component]
fn WorkoutCard(workout: Workout) -> impl IntoView {
    let (hovered, set_hovered) = create_signal(false);

    let badge_class = format!(
        "inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium {} {}",
        workout.bg_color, workout.text_color
    );

    view! {
        <div
            class="group relative bg-white rounded-lg overflow-hidden shadow-lg transition-all duration-300 hover:shadow-xl"
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            <div class="relative h-48 overflow-hidden bg-gradient-to-br from-gray-200 to-gray-300">
                <div class="absolute inset-0 flex items-center justify-center">
                    <svg class="w-24 h-24 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                </div>
                <div class={move || if hovered.get() { "absolute inset-0 bg-black bg-opacity-30 flex items-center justify-center opacity-100 transition-opacity duration-300" } else { "absolute inset-0 bg-black bg-opacity-30 flex items-center justify-center opacity-0 transition-opacity duration-300" }}>
                    <div class="bg-red-600 p-4 rounded-full opacity-90">
                        <svg class="text-white w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M8 5v14l11-7z" />
                        </svg>
                    </div>
                </div>
            </div>
            <div class="p-6">
                <div class="flex items-center">
                    <span class={badge_class}>
                        {workout.category}
                    </span>
                    <span class="ml-2 text-sm text-gray-500">{workout.duration}</span>
                </div>
                <a href="#" class="mt-4 block">
                    <h3 class="text-xl font-semibold text-slate-800">
                        {workout.title}
                    </h3>
                    <p class="mt-3 text-base text-gray-500">
                        {workout.description}
                    </p>
                </a>
                <div class="mt-6 flex items-center">
                    <div class="flex-shrink-0">
                        <div class="h-10 w-10 rounded-full bg-gradient-to-br from-red-400 to-red-600"></div>
                    </div>
                    <div class="ml-3">
                        <p class="text-sm font-medium text-gray-900">
                            {workout.coach}
                        </p>
                        <div class="flex space-x-1 text-sm text-gray-500 items-center">
                            <span>{format!("{:.1}", workout.rating)}</span>
                            <svg class="text-yellow-400 w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
                            </svg>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressTracking() -> impl IntoView {
    view! {
        <div class="py-12 bg-gray-100">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="lg:grid lg:grid-cols-2 lg:gap-8 lg:items-center">
                    <div>
                        <h2 class="text-3xl font-extrabold text-slate-800 sm:text-4xl">
                            "Track Your " <span class="text-red-600">"Progress"</span>
                        </h2>
                        <p class="mt-3 max-w-3xl text-lg text-gray-500">
                            "Our smart tracking system helps you stay on top of your fitness goals with detailed analytics and visual progress reports."
                        </p>
                        <div class="mt-8">
                            <div class="inline-flex rounded-md shadow">
                                <a href="#" class="inline-flex items-center justify-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-white bg-red-600 hover:bg-red-700">
                                    "Start Tracking"
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="mt-8 lg:mt-0">
                        <ProgressCard />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressCard() -> impl IntoView {
    view! {
        <div class="relative mx-auto max-w-md px-4 sm:max-w-3xl sm:px-6 lg:px-0">
            <div class="relative bg-white shadow-xl rounded-2xl p-8">
                <div class="flex justify-between items-center mb-6">
                    <div>
                        <h3 class="text-lg font-medium text-slate-800">"Your Weekly Progress"</h3>
                        <p class="text-sm text-gray-500">"Last updated 2 hours ago"</p>
                    </div>
                    <button class="p-2 rounded-full bg-gray-100 text-slate-800 hover:bg-gray-200">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                        </svg>
                    </button>
                </div>
                <div class="grid grid-cols-3 gap-4">
                    <ProgressRing percentage=85 label="Workouts" />
                    <ProgressRing percentage=72 label="Nutrition" />
                    <ProgressRing percentage=93 label="Recovery" />
                </div>
                <div class="mt-6">
                    <div class="flex items-center justify-between text-sm text-gray-500">
                        <span>"Weekly Goal"</span>
                        <span>"5/7 days"</span>
                    </div>
                    <div class="mt-1 w-full bg-gray-200 rounded-full h-2.5">
                        <div class="bg-red-600 h-2.5 rounded-full" style="width: 71%"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProgressRing(percentage: u32, label: &'static str) -> impl IntoView {
    let circumference = 100.0;
    let offset = circumference - (percentage as f32 / 100.0 * circumference);

    view! {
        <div class="text-center">
            <div class="mx-auto h-24 w-24 relative">
                <svg class="w-full h-full" viewBox="0 0 36 36" style="transform: rotate(-90deg);">
                    <path
                        d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                        fill="none"
                        stroke="#e2e8f0"
                        stroke-width="3"
                    />
                    <path
                        d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
                        fill="none"
                        stroke="#e11d48"
                        stroke-width="3"
                        stroke-dasharray={format!("{}, 100", percentage)}
                        style="transition: stroke-dashoffset 0.35s;"
                    />
                </svg>
                <div class="absolute inset-0 flex items-center justify-center">
                    <span class="text-lg font-bold text-slate-800">{format!("{}%", percentage)}</span>
                </div>
            </div>
            <h4 class="mt-2 text-sm font-medium text-slate-800">{label}</h4>
        </div>
    }
}

#[component]
fn Testimonials() -> impl IntoView {
    let testimonials = vec![
        (
            "Michael R.",
            "The Power Builder program transformed my strength. I added 50lbs to my deadlift in just 12 weeks!",
        ),
        (
            "Sarah J.",
            "Fat Burn Blast helped me lose 15 pounds while maintaining muscle. The workouts are challenging but fun!",
        ),
        (
            "David L.",
            "The mobility program fixed my chronic back pain. I can finally squat without discomfort!",
        ),
    ];

    view! {
        <div class="bg-white py-12">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="lg:text-center">
                    <h2 class="text-base text-red-600 font-semibold tracking-wide uppercase">"Testimonials"</h2>
                    <p class="mt-2 text-3xl leading-8 font-extrabold tracking-tight text-slate-800 sm:text-4xl">
                        "Athlete Success Stories"
                    </p>
                </div>
                <div class="mt-10">
                    <div class="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3">
                        <For
                            each=move || testimonials.clone()
                            key=|t| t.0.to_string()
                            children=move |(name, quote)| {
                                view! { <TestimonialCard name=name quote=quote /> }
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn TestimonialCard(name: &'static str, quote: &'static str) -> impl IntoView {
    view! {
        <div class="bg-gray-50 p-6 rounded-lg">
            <div class="flex items-center">
                <div class="flex-shrink-0">
                    <div class="h-12 w-12 rounded-full bg-gradient-to-br from-red-400 to-red-600"></div>
                </div>
                <div class="ml-4">
                    <h4 class="text-sm font-bold text-slate-800">{name}</h4>
                    <div class="flex mt-1">
                        {(0..5).map(|_| view! {
                            <svg class="text-yellow-400 w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
                            </svg>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
            <div class="mt-4">
                <p class="text-base text-gray-600">
                    {quote}
                </p>
            </div>
        </div>
    }
}

#[component]
fn CallToAction() -> impl IntoView {
    view! {
        <div class="bg-red-600">
            <div class="max-w-2xl mx-auto text-center py-16 px-4 sm:py-20 sm:px-6 lg:px-8">
                <h2 class="text-3xl font-extrabold text-white sm:text-4xl">
                    <span class="block">"Ready to transform your fitness?"</span>
                    <span class="block">"Start your journey today."</span>
                </h2>
                <p class="mt-4 text-lg leading-6 text-red-100">
                    "Join thousands of athletes who have already unlocked their potential with IronPulse."
                </p>
                <a href="#" class="mt-8 w-full inline-flex items-center justify-center px-5 py-3 border border-transparent text-base font-medium rounded-md text-red-600 bg-white hover:bg-gray-50 sm:w-auto">
                    "Get Your Free Plan"
                </a>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-slate-800 text-white">
            <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                    <FooterColumn title="Training" links=vec!["Programs", "Workouts", "Exercises"] />
                    <FooterColumn title="Nutrition" links=vec!["Meal Plans", "Recipes", "Macro Calculator"] />
                    <FooterColumn title="Community" links=vec!["Forums", "Challenges", "Coaches"] />
                    <FooterColumn title="Company" links=vec!["About", "Contact", "Careers"] />
                </div>
                <div class="mt-12 border-t border-gray-700 pt-8">
                    <div class="md:flex md:items-center md:justify-between">
                        <div class="flex justify-center md:order-2 space-x-6">
                            <SocialIcon d="M18 2h-3a5 5 0 00-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 011-1h3z" />
                            <SocialIcon d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zm0-2.163c-3.259 0-3.667.014-4.947.072-4.358.2-6.78 2.618-6.98 6.98-.059 1.281-.073 1.689-.073 4.948 0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98 1.281.058 1.689.072 4.948.072 3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98-1.281-.059-1.69-.073-4.949-.073zm0 5.838c-3.403 0-6.162 2.759-6.162 6.162s2.759 6.163 6.162 6.163 6.162-2.759 6.162-6.163c0-3.403-2.759-6.162-6.162-6.162zm0 10.162c-2.209 0-4-1.79-4-4 0-2.209 1.791-4 4-4s4 1.791 4 4c0 2.21-1.791 4-4 4zm6.406-11.845c-.796 0-1.441.645-1.441 1.44s.645 1.44 1.441 1.44c.795 0 1.439-.645 1.439-1.44s-.644-1.44-1.439-1.44z" />
                            <SocialIcon d="M23 3a10.9 10.9 0 01-3.14 1.53 4.48 4.48 0 00-7.86 3v1A10.66 10.66 0 013 4s-4 9 5 13a11.64 11.64 0 01-7 2c9 5 20 0 20-11.5a4.5 4.5 0 00-.08-.83A7.72 7.72 0 0023 3z" />
                            <SocialIcon d="M23 7l-7 5 7 5V7z M15 7v10H4V7h11z" />
                        </div>
                        <p class="mt-8 text-base text-gray-400 md:mt-0 md:order-1">
                            "© 2023 IronPulse. All rights reserved."
                        </p>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn FooterColumn(title: &'static str, links: Vec<&'static str>) -> impl IntoView {
    view! {
        <div>
            <h3 class="text-sm font-semibold uppercase tracking-wider">{title}</h3>
            <ul class="mt-4 space-y-4">
                <For
                    each=move || links.clone()
                    key=|link| link.to_string()
                    children=move |link| {
                        view! {
                            <li>
                                <a href="#" class="text-base text-gray-300 hover:text-white">{link}</a>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn SocialIcon(d: &'static str) -> impl IntoView {
    view! {
        <a href="#" class="text-gray-400 hover:text-white">
            <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24">
                <path d=d />
            </svg>
        </a>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

