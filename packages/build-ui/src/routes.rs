use bdk::prelude::*;

#[allow(unused)]
use crate::pages::*;
#[derive(Clone, Routable)]
#[rustfmt::skip]
#[derive(PartialEq)]
pub enum Route {
    #[nest("/:lang")]
        #[layout(HeaderLayout)]
            #[route("/")]
            RootPage { lang: Language},
        #[end_layout]
        #[nest("/agits/:agit_id")]
            #[layout(NavigationLayout)]
                #[route("/")]
                     HomePage { lang: Language, agit_id: i64 },
                #[nest("/orders")]
                    #[route("/sales-request")]
                        SalesRequestPage { lang: Language, agit_id: i64 },
                    #[route("/shipping-label")]
                        ShippingLabelPage { lang: Language, agit_id: i64 },
                #[end_nest]
                #[nest("/management")]
                    #[nest("/artworks")]
                        #[route("/")]
                            ArtworkPage { lang: Language, agit_id: i64 },
                        // #[route("/:artwork_id")]
                        //     ArtworkDetailPage { lang: Language, agit_id: i64, artwork_id: i64 },
                        #[route("/create")]
                            CreateArtworkPage { lang: Language, agit_id: i64 },
                        // #[route("/:artwork_id/edit")]
                        //     EditArtworkPage { lang: Language, agit_id: i64, artwork_id: i64 },
                    #[end_nest]
                    #[nest("/collections")]
                        #[route("/collection")]
                            CollectionPage { lang: Language, agit_id: i64 },
                        #[route("/:collection_id")]
                            CollectionDetailPage { lang: Language, agit_id: i64, collection_id:i64 },
                    #[end_nest] 
                    #[nest("/artists")]
                        #[route("/")]
                            ArtistPage { lang: Language, agit_id: i64 },    
                        #[route("/:artist_id")]
                            ArtistDetailPage { lang: Language, agit_id: i64, artist_id: i64 },
                        #[route("/create")]
                            CreateArtistPage { lang: Language, agit_id: i64},    
                        #[route("/:artist_id/edit")]
                            EditArtistPage { lang: Language, agit_id: i64, artist_id: i64 },
                        
                    #[end_nest] 
                    #[nest("/collectors")]
                        #[route("/")]  
                            CollectorPage { lang: Language, agit_id: i64 },
                        #[route("/:collector_id")]
                            CollectorDetailPage { lang: Language, agit_id: i64, collector_id: i64 },
                    #[end_nest]
                #[end_nest]

                #[nest("/hub")]
                    #[route("/dao")]   
                        DaoPage { lang: Language, agit_id: i64 },
                    #[route("/oracle")]   
                        OraclePage { lang: Language, agit_id: i64 },
                    #[route("/faq")]
                    FaqPage { lang: Language, agit_id: i64 },
                #[end_nest]

                #[nest("/analytics")]
                    #[route("/traffic")]
                        TrafficPage { lang: Language, agit_id: i64 },
                    #[route("/report")]  
                        ReportPage { lang: Language, agit_id: i64 },
                #[end_nest]

                #[route("/design")]  
                    DesignPage { lang: Language, agit_id: i64 },
                #[route("/extension-tool")]
                    ExtensionToolPage { lang: Language, agit_id: i64 },
            #[end_layout]
        #[end_nest]
    #[end_nest]
    #[redirect("/", || Route::RootPage { lang: Language::En })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
