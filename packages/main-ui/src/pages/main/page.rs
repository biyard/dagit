use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;

use crate::utils::format_number::format_number;
#[component]
pub fn PillButton(
    #[props(default = "".to_string())] class: String,
    text: String,
    selected: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class =  format!("{} border border-black rounded-full justify-center items-center px-4 py-1 text-base font-normal leading-normal tracking-wide {}",
        class,
        if selected {
            "bg-black text-white"
        } else {
            ""
        }
    );
    rsx! {
        button { class, onclick, {text} }
    }
}

#[component]
pub fn CategorySelector(options: Vec<String>, onchange: EventHandler<usize>) -> Element {
    let mut selected: Signal<usize> = use_signal(|| 0);
    use_effect(move || {
        onchange.call(selected());
    });
    rsx! {
        div { class: "flex flex-row gap-2.5",
            for (i , option) in options.into_iter().enumerate() {
                PillButton {
                    text: option.clone(),
                    selected: selected() == i,
                    onclick: move |_e| {
                        selected.set(i);
                    },
                }
            }
        

        }
    }
}

#[component]
pub fn Discover(
    category: String,
    agit_description: String,
    artist_name: String,
    artist_description: String,
) -> Element {
    rsx! {
        div { class: "grid grid-cols-12 grid-rows-1 gap-6 relative",
            div { class: "absolute z-10 w-1/4 px-8 py-10 mt-10 bg-white text-black",
                div {
                    p { class: "self-stretch text-base font-semibold uppercase leading-normal tracking-wide",
                        "{category}"
                    }
                    p { class: "self-stretch text-3xl font-semibold pb-8", "About Our Agit" }
                    p { class: "text-[#6b6b6b] text-base font-light leading-normal tracking-wide",
                        "{agit_description}"
                        ""
                    }
                }
            }
            div { class: "col-start-2 col-span-7 h-116 relative",
                img {
                    class: "w-full h-full object-cover",
                    src: "https://loremflickr.com/400/390?random=1",
                }
                button { class: "absolute inset-0 flex items-center justify-center text-white text-3xl",
                    "▶"
                }
            }
            div { class: "col-span-4 flex flex-col gap-2.5",
                img {
                    class: "w-full h-73 object-cover",
                    src: "https://loremflickr.com/400/390?random=10",
                }

                div {
                    p { class: "text-black text-base font-semibold uppercase leading-normal tracking-wide",
                        "ARTIST"
                    }
                    p { class: "text-black text-2xl font-medium", "{artist_name}" }
                
                }
                div { class: "text-[#6b6b6b] text-base font-light leading-normal tracking-wide",
                    "{artist_description}"
                }
            }
        
        }
    }
}

#[component]
pub fn MainPage(lang: Language) -> Element {
    let ctrl = super::controller::Controller::init()?;
    rsx! {
        div { class: "flex flex-col w-full max-w-[1440px] px-5 self-center",
            article { class: "grid grid-cols-12 relative",
                div { class: "absolute z-10 w-1/3 flex flex-col gap-3 px-14 py-10 mt-15 bg-white text-black",
                    h1 { class: "text-black text-6xl font-black", "Here is the Agit" }
                    p { class: "text-[#6b6b6b] text-xl font-semibold leading-loose tracking-wide",
                        "Portland OR, Jan 19, 2025"
                    }
                    p { class: "text-[#6b6b6b] text-sm font-light leading-normal tracking-wide",
                        "Yihong Hsu's artwork primarily focuses on the elements of nature, which she uses to depict stories and experiences. Her paintings often"
                    }
                    button { class: "w-fit bg-black text-white px-10 py-4 text-base font-semibold",
                        "View Gallery"
                    }
                }
                div { class: "col-start-2 col-span-10 h-160 mb-15",
                    img {
                        class: "w-full h-full object-cover",
                        src: "https://loremflickr.com/400/390?random=1",
                    }
                
                }
            }
            article { class: "grid grid-cols-12 gap-6 [&>div>div>img]:h-[390px]",
                div { class: "col-span-5 ",
                    ArtworkCard {
                        artwork: Artwork {
                            image: "https://loremflickr.com/400/390?random=97".to_string(),
                            title: "Title".to_string(),
                            tag: "Tag".to_string(),
                            artist: "Artist Name".to_string(),
                            price: "$1,400".to_string(),
                        },
                    }
                }
                div { class: "col-span-3",
                    ArtworkCard {
                        artwork: Artwork {
                            image: "https://loremflickr.com/400/390?random=98".to_string(),
                            title: "Title".to_string(),
                            tag: "Tag".to_string(),
                            artist: "Artist Name".to_string(),
                            price: "$1,400".to_string(),
                        },
                    }
                }
                div { class: "col-span-4",
                    ArtworkCard {
                        artwork: Artwork {
                            image: "https://loremflickr.com/400/390?random=99".to_string(),
                            title: "Title".to_string(),
                            tag: "Tag".to_string(),
                            artist: "Artist Name".to_string(),
                            price: "$1,400".to_string(),
                        },
                    }
                }
            
            }
            article { class: "flex flex-col gap-30 w-full",
                Section {
                    title: "proposal".to_string(),
                    on_view_all_clicked: move |e: MouseEvent| {
                        tracing::debug!("Proposal {:?}", e);
                    },
                    CategorySelector {
                        options: vec!["Drawing".to_string(), "Painting".to_string(), "Sculpture".to_string()],
                        onchange: move |category: usize| {
                            tracing::debug!("Category {:?}", category);
                        },
                    }
                    div { class: "snap-x w-full overflow-x-auto pt-6 pb-15 flex",
                        for image in ctrl.get_images().into_iter() {
                            ProposalSlide {
                                image: image.url,
                                agit_name: image.agit_name,
                                agit_location: image.location,
                                title: image.title,
                                description: image.description,
                                votes: image.votes,
                                followers: image.followers,
                            }
                        }
                    }
                }
                Section { title: "discover".to_string(),
                    Discover {
                        category: "category",
                        agit_description: "We visited Jaclyn Conley in her studio outside New Haven and met an artist who wanders between the grand masters of painting and her contemporary images of reflection and silent protest.",
                        artist_name: "Jaclyn Conley".to_string(),
                        artist_description: "Honey is a free browser extension that can find deals for you when you shop online. It scans our database of all the best promo codes and coupon codes on the web and then applies them to your cart at check-out...",
                    }
                }
                Section { title: "editorial".to_string(),
                    EditorialMainArticle {
                        article: Article {
                            image: "https://loremflickr.com/400/390?random=1".to_string(),
                            category: "CATEGORY".to_string(),
                            title: "This is the Article Title".to_string(),
                            overview: "Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society."
                                .to_string(),
                            created_at: "Oct 20, 2025".to_string(),
                            author: "Chris. J".to_string(),
                            author_image: "https://loremflickr.com/400/390?random=1".to_string(),
                            author_affiliation: "New Foundation Curator".to_string(),
                        },
                    }
                    EditorialSubArticle {
                        articles: vec![
                            Article {
                                image: "https://loremflickr.com/400/390?random=1".to_string(),
                                category: "CATEGORY".to_string(),
                                title: "This is the Article Title".to_string(),
                                overview: "Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society."
                                    .to_string(),
                                created_at: "Oct 20, 2025".to_string(),
                                author: "Chris. J".to_string(),
                                author_image: "https://loremflickr.com/400/390?random=1".to_string(),
                                author_affiliation: "New Foundation Curator".to_string(),
                            },
                            Article {
                                image: "https://loremflickr.com/400/600?random=1".to_string(),
                                category: "CATEGORY".to_string(),
                                title: "This is the Article Title".to_string(),
                                overview: "Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society."
                                    .to_string(),
                                created_at: "Oct 20, 2025".to_string(),
                                author: "Chris. J".to_string(),
                                author_image: "https://loremflickr.com/400/390?random=1".to_string(),
                                author_affiliation: "New Foundation Curator".to_string(),
                            },
                            Article {
                                image: "https://loremflickr.com/400/700?random=1".to_string(),
                                category: "CATEGORY".to_string(),
                                title: "This is the Article Title".to_string(),
                                overview: "Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society."
                                    .to_string(),
                                created_at: "Oct 20, 2025".to_string(),
                                author: "Chris. J".to_string(),
                                author_image: "https://loremflickr.com/400/390?random=1".to_string(),
                                author_affiliation: "New Foundation Curator".to_string(),
                            },
                        ],
                    }
                }
                Section { title: "fair & event".to_string(),
                    div { class: "grid grid-cols-12 gap-6",
                        img {
                            class: "col-span-12 w-full",
                            src: "https://loremflickr.com/1440/633",
                        }
                        div { class: "col-span-6 text-black",
                            p { class: " text-base font-semibold uppercase leading-normal tracking-wide",
                                "CURRENT SHOW"
                            }
                            p { class: "pt-2 pb-5 text-6xl font-semibold", "Show Title" }
                            p { class: "pb-10 text-[#6b6b6b] text-xl font-semibold leading-loose tracking-wide",
                                "Portland OR, Jan 19, 2025"
                            }
                            div { class: "flex flex-row gap-3 items-end",
                                div { class: "h-14 w-14 bg-black" }
                                div { class: "flex flex-col gap-1",
                                    p { class: "text-[#282828] text-base font-medium",
                                        "Gallery Name"
                                    }
                                    p { class: "opacity-60 text-[#282828] text-sm font-semibold leading-tight tracking-wide",
                                        "Portland OR"
                                    }
                                }
                                PillButton {
                                    class: "px-8 py-2.5 ml-20",
                                    text: "Follow",
                                    selected: false,
                                    onclick: move |_e| {
                                        tracing::debug!("Follow Clicked");
                                    },
                                }
                            }
                        }
                        div { class: "col-span-6 p-2.5 pt-8 text-[#6b6b6b] text-base font-light leading-normal tracking-wide",
                            "Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society. Explore powerful artworks that amplify voices for equality, diversity, and justice. This collection brings together artists who use their creativity to spark conversations and inspire change in our society"
                        }
                        div { class: "col-span-12 w-full snap-x overflow-x-auto pt-25",
                            div { class: "flex mb-15",
                                for image in ctrl.get_images().into_iter() {
                                    div { class: "snap-start w-1/6 pr-6 shrink-0",
                                        ArtworkCard {
                                            artwork: Artwork {
                                                image: image.url.clone(),
                                                title: "Title".to_string(),
                                                tag: "Tag".to_string(),
                                                artist: "Artist Name".to_string(),
                                                price: "$1,400".to_string(),
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    
                    }
                }
                Section {
                    title: "upcoming".to_string(),
                    on_view_all_clicked: move |e: MouseEvent| {
                        tracing::debug!("Upcoming {:?}", e);
                    },
                    div { class: "snap-x w-full overflow-x-auto",
                        div { class: "flex mb-15",
                            for image in ctrl.get_images().into_iter() {
                                UpcomingSlide {
                                    image: image.url,
                                    gallery_name: image.agit_name,
                                    gallery_location: image.location,
                                    title: image.title,
                                    description: image.description,
                                    _gallery_logo: "".to_string(),
                                    schedule: "Jan 13 - Feb 21, 2025".to_string(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Section(
    class: Option<String>,
    title: String,
    on_view_all_clicked: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        section { class: "even:[&>div]:flex-row-reverse",
            div { class: "flex flex-row items-end justify-between mb-6",
                h1 { class: "text-8xl font-black text-[#E6E6E6] uppercase", {title} }
                if on_view_all_clicked.is_some() {
                    p { class: "cursor-pointer uppercase underline", "view all" }
                }
            }
            div { {children} }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub image: String,
    pub category: String, //TODO: USE Enum
    pub title: String,
    pub overview: String,
    pub created_at: String,

    pub author: String,
    pub author_image: String,
    pub author_affiliation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artwork {
    pub image: String,
    pub title: String,
    pub tag: String,
    pub artist: String,
    pub price: String,
}

#[component]
pub fn ArtworkCard(artwork: Artwork) -> Element {
    rsx! {
        div { class: "w-full flex flex-col",
            img { class: "w-full object-cover", src: artwork.image.clone() }
            div { class: "flex flex-col",
                div { class: "flex flex-row gap-2 pt-2.5 pb-1",

                    p { class: "text-xl font-light text-black leading-normal tracking-wide",
                        "{artwork.title}"
                    }
                    p { class: "flex px-2 border border-black justify-center items-center text-black text-xs font-medium leading-normal tracking-wide",
                        "{artwork.tag}"
                    }
                }

                p { class: "pt-1 pb-2 text-[#6b6b6b] text-sm font-semibold leading-tight tracking-wide",
                    "by {artwork.artist}"
                }
                p { class: "text-base font-bold leading-normal tracking-wide", "{artwork.price}" }
            }
        
        }
    }
}

#[component]
pub fn EditorialSubArticle(articles: Vec<Article>) -> Element {
    rsx! {
        div { class: "grid grid-cols-12 grid-flow-row gap-6 ",
            for article in articles.into_iter() {
                div { class: "col-span-4 gap-2.5",
                    img {
                        class: "w-full aspect-[4/3] object-cover",
                        src: article.image.clone(),
                    }
                    div { class: "flex flex-col text-black  tracking-wide pt-2.5",
                        p { class: "text-base font-semibold uppercase leading-normal",
                            "{article.category}"
                        }
                        p { class: "text-xl font-light leading-normal pt-1 pb-2",
                            "{article.title}"
                        }
                        p { class: "self-stretch text-[#6b6b6b] text-sm font-semibold leading-tight",
                            "by {article.author}"
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn EditorialMainArticle(article: Article) -> Element {
    rsx! {
        div { class: "w-full grid grid-flow-col grid-cols-12 text-black gap-6 mb-[27px]",
            div { class: "col-span-8 object-cover",
                img { class: "w-full", src: article.image.clone() }
            }
            div { class: "col-span-4 col-start-9 flex flex-col self-center",
                div { class: "text-[15px] font-semibold uppercase leading-[25px] tracking-wide",
                    "{article.category}"
                }
                div { class: "text-6xl font-semibold mt-2 mb-9", "{article.title}" }
                div { class: "text-base font-light leading-normal tracking-wide", "{article.overview}" }
                div { class: "text-[#6b6b6b] text-sm font-semibold mt-4", "{article.created_at}" }
                div { class: "flex flex-row mt-2 justify-end items-end text-end",
                    div { class: "flex flex-col gap-1 mr-3",
                        p { class: "font-medium", "{article.author}" }
                        p { class: "opacity-60 text-[#282828]", "{article.author_affiliation}" }
                    }
                    img {
                        class: "w-[60px] h-[60px] object-cover rounded-full",
                        src: article.author_image.clone(),
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProposalSlide(
    image: String,
    agit_name: String,
    agit_location: String,
    title: String,
    description: String,
    votes: i64,
    followers: i64,
) -> Element {
    rsx! {
        div { class: "snap-start w-1/4 pr-6 shrink-0",
            div { class: "w-full h-full flex flex-col gap-2.5",
                img {
                    class: "w-full h-[220px] object-cover",
                    src: image,
                    alt: title.clone(),
                }
                div { class: "flex flex-row gap-2.5",
                    div { class: "aspect-square bg-black h-full" }
                    div { class: "flex flex-col gap-1",
                        p { class: "text-black text-base font-normal", {agit_name} }
                        p { class: "opacity-60 text-[#282828] text-sm font-semibold",
                            {agit_location}
                        }
                    }
                }
                div { class: "flex flex-col gap-1",
                    p { class: "text-black text-lg font-semibold", {title.clone()} }
                    p { class: "text-[#6b6b6b] text-[15px] font-light", {description} }
                }
                button { class: "border border-black justify-center items-center w-full p-2 text-black text-[15px] font-normal font-['Pretendard'] leading-[25px] tracking-wide ",
                    "Vote"
                }
                div { class: "text-black text-sm font-normal inline-flex gap-4",
                    p {
                        span { class: "font-semibold", "{format_number(votes)}" }
                        " Votes"
                    }
                    p {
                        span { class: "font-semibold", "{format_number(followers)}" }
                        " Followers"
                    }
                }
            }
        }
    }
}

#[component]
pub fn UpcomingSlide(
    image: String,
    title: String,
    schedule: String,
    description: String,
    gallery_name: String,
    gallery_location: String,
    _gallery_logo: String,
) -> Element {
    rsx! {
        div { class: "snap-start w-1/4 pr-6 shrink-0",
            div { class: "w-full h-full flex flex-col",
                img {
                    class: "w-full h-[320px] object-cover",
                    src: image,
                    alt: title.clone(),
                }
                div { class: "flex flex-col pt-2.5 pb-5",
                    p { class: "text-black text-xl font-normal pb-1", {title.clone()} }
                    p { class: "text-[#6b6b6b] text-sm font-semibold leading-tight tracking-wide",
                        {schedule}
                    }
                    p { class: "text-[#6b6b6b] text-[15px] font-light py-5", {description} }
                
                }
                div { class: "flex flex-row gap-2.5",
                    div { class: "aspect-square bg-black h-full" }
                    div { class: "flex flex-col gap-1",
                        p { class: "text-black text-base font-normal", {gallery_name} }
                        p { class: "opacity-60 text-[#282828] text-sm font-semibold",
                            {gallery_location}
                        }
                    }
                }
            


            }
        }
    }
}
