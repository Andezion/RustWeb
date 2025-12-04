use leptos::*;
use crate::api::{get_topics, create_topic, get_posts, create_post, CreateTopicRequest, CreatePostRequest, ForumTopic};
use crate::components::card::Card;

#[component]
pub fn ForumPage() -> impl IntoView {
    let (topics, set_topics) = create_signal(Vec::<ForumTopic>::new());
    let (selected_topic, set_selected_topic) = create_signal(None::<String>);
    let (posts, set_posts) = create_signal(Vec::new());
    let (new_topic_title, set_new_topic_title) = create_signal(String::new());
    let (new_post_content, set_new_post_content) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());

    create_effect(move |_| {
        spawn_local(async move {
            match get_topics().await {
                Ok(topics_data) => set_topics.set(topics_data),
                Err(e) => set_message.set(format!("Failed to load topics: {}", e)),
            }
        });
    });

    let handle_create_topic = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let title = new_topic_title.get();
        
        spawn_local(async move {
            match create_topic(CreateTopicRequest { title: title.clone() }).await {
                Ok(_) => {
                    set_new_topic_title.set(String::new());
                    set_message.set("Topic created!".to_string());
                    if let Ok(topics_data) = get_topics().await {
                        set_topics.set(topics_data);
                    }
                }
                Err(e) => set_message.set(format!("Failed to create topic: {}", e)),
            }
        });
    };

    let load_posts = move |topic_id: String| {
        set_selected_topic.set(Some(topic_id.clone()));
        spawn_local(async move {
            match get_posts(&topic_id).await {
                Ok(posts_data) => set_posts.set(posts_data),
                Err(e) => set_message.set(format!("Failed to load posts: {}", e)),
            }
        });
    };

    let handle_create_post = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if let Some(topic_id) = selected_topic.get() {
            let content = new_post_content.get();
            
            spawn_local(async move {
                match create_post(CreatePostRequest { topic_id: topic_id.clone(), content }).await {
                    Ok(_) => {
                        set_new_post_content.set(String::new());
                        set_message.set("Post created!".to_string());
                        if let Ok(posts_data) = get_posts(&topic_id).await {
                            set_posts.set(posts_data);
                        }
                    }
                    Err(e) => set_message.set(format!("Failed to create post: {}", e)),
                }
            });
        }
    };

    view! {
        <div class="forum-page">
            <h1>"Forum"</h1>

            {move || {
                let msg = message.get();
                if !msg.is_empty() {
                    view! {
                        <div class="message">{msg}</div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}

            <div class="forum-content">
                <div class="topics-section">
                    <Card title="Create Topic">
                        <form on:submit=handle_create_topic>
                            <input
                                type="text"
                                placeholder="Topic title"
                                on:input=move |ev| set_new_topic_title.set(event_target_value(&ev))
                                prop:value=new_topic_title
                            />
                            <button type="submit">"Create Topic"</button>
                        </form>
                    </Card>

                    <Card title="Topics">
                        <ul class="topics-list">
                            {move || topics.get().into_iter().map(|topic| {
                                let topic_id = topic.id.clone();
                                view! {
                                    <li 
                                        class="topic-item"
                                        on:click=move |_| load_posts(topic_id.clone())
                                    >
                                        <h4>{topic.title}</h4>
                                        <small>{topic.created_at}</small>
                                    </li>
                                }
                            }).collect::<Vec<_>>()}
                        </ul>
                    </Card>
                </div>

                <div class="posts-section">
                    {move || {
                        if selected_topic.get().is_some() {
                            view! {
                                <div>
                                    <Card title="New Post">
                                        <form on:submit=handle_create_post>
                                            <textarea
                                                placeholder="Write your post..."
                                                on:input=move |ev| set_new_post_content.set(event_target_value(&ev))
                                                prop:value=new_post_content
                                            />
                                            <button type="submit">"Post"</button>
                                        </form>
                                    </Card>

                                    <Card title="Posts">
                                        <ul class="posts-list">
                                            {move || posts.get().into_iter().map(|post| {
                                                view! {
                                                    <li class="post-item">
                                                        <p>{post.content}</p>
                                                        <small>{post.created_at}</small>
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </Card>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <p>"Select a topic to view posts"</p>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
